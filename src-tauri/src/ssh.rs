//! `ssh` の定義・実行・出力パース。管理者権限は不要。
//! 対話的なログインや任意のリモートコマンド実行は扱わない。
//! GUIからは安全な読み取り系だけを許可する:
//! - `config`: `ssh -T -G host` (接続せず有効な設定をダンプ)
//! - `test`: `BatchMode=yes` + `ConnectTimeout` + 固定コマンド `exit` の疎通確認
//!   (パスワードプロンプトを出さず、PTYも使わない)
//! 鍵ファイル指定 (`-i`)・設定ファイル指定 (`-F`)・ProxyJump (`-J`)・
//! エージェント転送 (`-A`) は受け付けない。
//! ポート転送 (`-L`/`-R`/`-D`) はトンネル管理 (`Tunnel`) でのみ扱う。
//! 既定の鍵・ssh-agentの設定がそのまま使われる。

use std::collections::HashMap;
use std::process::{Child, Stdio};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Mutex, OnceLock,
};

use serde::{Deserialize, Serialize};

use crate::privileged;

/// 問い合わせモード。
pub const MODE_CONFIG: &str = "config";
pub const MODE_TEST: &str = "test";
/// 許可する StrictHostKeyChecking の値。`no` (検証無効化) は受け付けない。
pub const ALLOWED_STRICT: &[&str] = &["ask", "accept-new", "yes"];

pub const DEFAULT_MODE: &str = MODE_CONFIG;
pub const DEFAULT_STRICT: &str = "accept-new";
pub const DEFAULT_TIMEOUT: u32 = 10;
pub const MIN_TIMEOUT: u32 = 5;
pub const MAX_TIMEOUT: u32 = 30;
pub const MIN_PORT: u32 = 1;
pub const MAX_PORT: u32 = 65535;
const MAX_HOST_LEN: usize = 253;

/// `ssh -G` でよく見るキー。Frontend の注目表示用。
/// パース自体は全キーを generic に扱う。
pub const FEATURED_KEYS: &[&str] = &[
    "hostname",
    "user",
    "port",
    "identityfile",
    "stricthostkeychecking",
    "batchmode",
    "connecttimeout",
    "proxyjump",
    "serveraliveinterval",
    "serveralivecountmax",
    "forwardagent",
    "identitiesonly",
];

/// Frontend の `getSsh` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SshQuery {
    pub host: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub port: Option<u32>,
    #[serde(default = "default_mode")]
    pub mode: String,
    #[serde(default = "default_timeout")]
    pub connect_timeout: u32,
    #[serde(default = "default_strict")]
    pub strict: String,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub transport: String,
}

fn default_mode() -> String {
    DEFAULT_MODE.to_string()
}

fn default_timeout() -> u32 {
    DEFAULT_TIMEOUT
}

fn default_strict() -> String {
    DEFAULT_STRICT.to_string()
}

/// `ssh -G` の1行。Frontend の `SshConfigEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct SshConfigEntry {
    pub key: String,
    pub value: String,
}

/// `get_ssh` の返却値。Frontend の `SshSnapshot` とフィールドを一致させること。
/// config モードでは `entries`、test モードでは `stdout` が中心になる。
#[derive(Debug, Clone, Serialize)]
pub struct SshSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub mode: String,
    pub entries: Vec<SshConfigEntry>,
    pub stdout: String,
    pub raw: String,
    pub stderr: String,
}

/// `ssh -G [user@]host` または疎通確認
pub struct Ssh {
    args: Vec<String>,
    mode: String,
}

impl Ssh {
    pub fn new(query: SshQuery) -> Result<Self, String> {
        let args = build_args(&query)?;
        Ok(Self {
            args,
            mode: query.mode.trim().to_string(),
        })
    }

    pub fn preview(&self) -> String {
        format!("ssh {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<SshSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/usr/bin/ssh", &refs, preview.clone())?;
        let stdout = output.stdout.trim_end().to_string();
        let entries = if self.mode == MODE_CONFIG {
            parse_config(&output.stdout)
        } else {
            Vec::new()
        };
        Ok(SshSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            mode: self.mode.clone(),
            entries,
            stdout: stdout.clone(),
            raw: stdout,
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: SshQuery) -> Result<SshSnapshot, String> {
    Ssh::new(query).and_then(|cmd| cmd.run())
}

fn build_args(query: &SshQuery) -> Result<Vec<String>, String> {
    let host = query.host.trim();
    let user = query.user.trim();
    let mode = query.mode.trim();
    let strict = query.strict.trim().to_lowercase();
    validate_host(host)?;
    validate_user(user)?;
    validate_mode(mode)?;
    validate_transport(&query.transport)?;
    if mode == MODE_TEST {
        validate_port(query.port)?;
        validate_timeout(query.connect_timeout)?;
        validate_strict(&strict)?;
    }

    let mut args: Vec<String> = Vec::new();
    if query.transport == "4" {
        args.push("-4".to_string());
    } else if query.transport == "6" {
        args.push("-6".to_string());
    }
    if query.verbose {
        args.push("-v".to_string());
    }
    if mode == MODE_CONFIG {
        // -T でPTY割り当て警告を抑止する (-G と併用可)
        args.push("-T".to_string());
        // ポートは -G でも有効な指定なので許可する
        if let Some(port) = query.port {
            validate_port(query.port)?;
            args.push("-p".to_string());
            args.push(port.to_string());
        }
        args.push("-G".to_string());
        args.push(destination(host, user));
    } else {
        // 疎通確認: パスワード入力を出さず、PTYも使わず、固定の `exit` だけを実行する
        args.push("-T".to_string());
        args.push("-o".to_string());
        args.push("BatchMode=yes".to_string());
        args.push("-o".to_string());
        args.push(format!("ConnectTimeout={}", query.connect_timeout));
        args.push("-o".to_string());
        args.push(format!("StrictHostKeyChecking={strict}"));
        if let Some(port) = query.port {
            args.push("-p".to_string());
            args.push(port.to_string());
        }
        args.push(destination(host, user));
        args.push("exit".to_string());
    }
    Ok(args)
}

fn destination(host: &str, user: &str) -> String {
    if user.is_empty() {
        host.to_string()
    } else {
        format!("{user}@{host}")
    }
}

/// ssh のオプション解釈を防ぐため、ホストをホスト名/IP相当に制限する。
/// 先頭 `-` 禁止。IPとして解釈できなければ英数字と `._-` のみ許可。
fn validate_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("ホストを入力してください".to_string());
    }
    if host.len() > MAX_HOST_LEN {
        return Err("ホストが長すぎます".to_string());
    }
    if host.starts_with('-') {
        return Err(format!("ホストが不正です: {host}"));
    }
    if host.parse::<std::net::IpAddr>().is_ok() {
        return Ok(());
    }
    // `user@host` 形式は user 欄に分けて入力させる
    if host.contains('@') || host.contains(' ') || host.contains('/') {
        return Err(format!("ホストが不正です: {host}"));
    }
    let ok = host
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-' | ':'));
    if !ok {
        return Err(format!("ホストが不正です: {host}"));
    }
    if host.contains("..") {
        return Err(format!("ホストが不正です: {host}"));
    }
    Ok(())
}

fn validate_user(user: &str) -> Result<(), String> {
    if user.is_empty() {
        return Ok(());
    }
    if user.len() > 32 {
        return Err(format!("ユーザ名が不正です: {user}"));
    }
    let mut chars = user.chars();
    let first = chars.next().unwrap();
    if !(first.is_ascii_alphanumeric() || first == '_') {
        return Err(format!("ユーザ名が不正です: {user}"));
    }
    if user.starts_with('-') {
        return Err(format!("ユーザ名が不正です: {user}"));
    }
    let ok = user
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    if !ok {
        return Err(format!("ユーザ名が不正です: {user}"));
    }
    Ok(())
}

fn validate_port(port: Option<u32>) -> Result<(), String> {
    match port {
        None => Ok(()),
        Some(p) if (MIN_PORT..=MAX_PORT).contains(&p) => Ok(()),
        Some(_) => Err(format!("ポートは {MIN_PORT}〜{MAX_PORT} の範囲で指定してください")),
    }
}

fn validate_timeout(timeout: u32) -> Result<(), String> {
    if (MIN_TIMEOUT..=MAX_TIMEOUT).contains(&timeout) {
        Ok(())
    } else {
        Err(format!(
            "接続タイムアウトは {MIN_TIMEOUT}〜{MAX_TIMEOUT} 秒の範囲で指定してください"
        ))
    }
}

fn validate_strict(strict: &str) -> Result<(), String> {
    if ALLOWED_STRICT.contains(&strict) {
        Ok(())
    } else {
        Err(format!("ホスト鍵の確認方法が不正です: {strict}"))
    }
}

fn validate_mode(mode: &str) -> Result<(), String> {
    if matches!(mode, MODE_CONFIG | MODE_TEST) {
        Ok(())
    } else {
        Err(format!("モードが不正です: {mode}"))
    }
}

fn validate_transport(transport: &str) -> Result<(), String> {
    if matches!(transport, "" | "auto" | "4" | "6") {
        Ok(())
    } else {
        Err(format!("トランスポートが不正です: {transport}"))
    }
}

/// `ssh -G` の `key value...` 形式をパースする。値は空白を含みうるので先頭2トークンで切る。
fn parse_config(stdout: &str) -> Vec<SshConfigEntry> {
    let mut out = Vec::new();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.splitn(2, char::is_whitespace);
        let Some(key) = parts.next() else {
            continue;
        };
        if key.is_empty() {
            continue;
        }
        let value = parts.next().unwrap_or("").trim().to_string();
        out.push(SshConfigEntry {
            key: key.to_string(),
            value,
        });
    }
    out
}

// ---- SSHトンネル (`-L` / `-R` / `-D`) の管理 ----
// `-N` (リモートコマンドなし) でフォアグラウンド起動し、子プロセスとして保持する。
// `-f` によるバックグラウンド化はしない (アプリが死んでも残る野良プロセスを作らないため)。
// アプリ終了時は `kill_all_tunnels` で全て停止する。
// バインド先はループバックのみ許可し、LANへの公開 (`0.0.0.0` 等) は受け付けない。

/// トンネルの方向。
pub const TUNNEL_LOCAL: &str = "local"; // -L: 手元ポート → 向こう側
pub const TUNNEL_REMOTE: &str = "remote"; // -R: 向こう側ポート → 手元
pub const TUNNEL_DYNAMIC: &str = "dynamic"; // -D: SOCKSプロキシ

/// 起動直後の成否判定の待ち時間。ポート競合などはここで検出する。
const STARTUP_GRACE_MS: u64 = 1500;
/// 許可するバインドアドレス。空欄は 127.0.0.1 として扱う。
const ALLOWED_BINDS: &[&str] = &["", "127.0.0.1", "localhost"];

/// Frontend の `startSshTunnel` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TunnelQuery {
    pub mode: String,
    #[serde(default)]
    pub local_host: String,
    pub local_port: u32,
    #[serde(default)]
    pub remote_host: String,
    #[serde(default)]
    pub remote_port: Option<u32>,
    pub host: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub port: Option<u32>,
    #[serde(default)]
    pub keepalive: bool,
    #[serde(default)]
    pub transport: String,
}

/// トンネル起動の返却値。Frontend の `TunnelStarted` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct TunnelStarted {
    pub id: String,
    pub command: String,
    pub summary: String,
}

/// トンネル一覧の1件。Frontend の `TunnelInfo` と一致させること (起動中のみ返す)。
#[derive(Debug, Clone, Serialize)]
pub struct TunnelInfo {
    pub id: String,
    pub command: String,
    pub summary: String,
    pub mode: String,
}

struct ManagedTunnel {
    command: String,
    summary: String,
    mode: String,
    child: Child,
}

fn tunnels() -> &'static Mutex<HashMap<String, ManagedTunnel>> {
    static TUNNELS: OnceLock<Mutex<HashMap<String, ManagedTunnel>>> = OnceLock::new();
    TUNNELS.get_or_init(|| Mutex::new(HashMap::new()))
}

static NEXT_TUNNEL_ID: AtomicU64 = AtomicU64::new(1);

/// トンネル用 `ssh -N ...` の引組み立て。プレビューと実行で共有する。
struct TunnelSpec {
    args: Vec<String>,
    summary: String,
    mode: String,
}

impl TunnelSpec {
    fn new(query: &TunnelQuery) -> Result<Self, String> {
        let mode = query.mode.trim();
        validate_tunnel_mode(mode)?;
        let bind = normalize_bind(query.local_host.trim())?;
        validate_port(Some(query.local_port))?;
        if mode != TUNNEL_DYNAMIC {
            validate_host(query.remote_host.trim())?;
            match query.remote_port {
                Some(port) => validate_port(Some(port))?,
                None => return Err("転送先ポートを入力してください".to_string()),
            }
        }
        validate_host(query.host.trim())?;
        validate_user(query.user.trim())?;
        validate_port(query.port)?;
        validate_transport(&query.transport)?;

        let mut args = vec![
            "-N".to_string(),
            "-T".to_string(),
            "-o".to_string(),
            "BatchMode=yes".to_string(),
            "-o".to_string(),
            "ExitOnForwardFailure=yes".to_string(),
        ];
        if query.keepalive {
            args.push("-o".to_string());
            args.push("ServerAliveInterval=30".to_string());
            args.push("-o".to_string());
            args.push("ServerAliveCountMax=3".to_string());
        }
        if query.transport == "4" {
            args.push("-4".to_string());
        } else if query.transport == "6" {
            args.push("-6".to_string());
        }
        if let Some(port) = query.port {
            args.push("-p".to_string());
            args.push(port.to_string());
        }
        let dest = destination(query.host.trim(), query.user.trim());
        let (flag, spec, summary) = match mode {
            TUNNEL_LOCAL => {
                let remote_port = query.remote_port.unwrap();
                let spec = format!(
                    "{bind}:{}:{}:{remote_port}",
                    query.local_port,
                    query.remote_host.trim()
                );
                let summary = format!(
                    "localhost:{} → {}:{remote_port} (via {dest})",
                    query.local_port,
                    query.remote_host.trim()
                );
                ("-L", spec, summary)
            }
            TUNNEL_REMOTE => {
                let remote_port = query.remote_port.unwrap();
                let spec = format!(
                    "{bind}:{}:{}:{remote_port}",
                    query.local_port,
                    query.remote_host.trim()
                );
                let summary = format!(
                    "{dest} の {remote_port} → localhost:{}",
                    query.local_port
                );
                ("-R", spec, summary)
            }
            _ => {
                let spec = format!("{bind}:{}", query.local_port);
                let summary = format!(
                    "SOCKS localhost:{} (via {dest})",
                    query.local_port
                );
                ("-D", spec, summary)
            }
        };
        args.push(flag.to_string());
        args.push(spec);
        args.push(dest);
        Ok(Self {
            args,
            summary,
            mode: mode.to_string(),
        })
    }

    fn preview(&self) -> String {
        format!("ssh {}", self.args.join(" "))
    }
}

/// トンネルを起動する。ポート競合など起動直後の失敗はここでエラーとして返す。
pub fn start_tunnel(query: TunnelQuery) -> Result<TunnelStarted, String> {
    let spec = TunnelSpec::new(&query)?;
    let preview = spec.preview();
    let refs: Vec<&str> = spec.args.iter().map(String::as_str).collect();
    let mut child = std::process::Command::new("/usr/bin/ssh")
        .args(&refs)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("トンネルの起動に失敗しました: {e}"))?;
    std::thread::sleep(std::time::Duration::from_millis(STARTUP_GRACE_MS));
    match child.try_wait() {
        Ok(Some(status)) => {
            let output = child
                .wait_with_output()
                .map(|o| String::from_utf8_lossy(&o.stderr).trim().to_string())
                .unwrap_or_default();
            let detail = if output.is_empty() {
                format!("exit code {}", status.code().unwrap_or(-1))
            } else {
                output
            };
            return Err(format!("トンネルの起動に失敗しました: {detail}"));
        }
        Ok(None) => {}
        Err(e) => {
            let _ = child.kill();
            return Err(format!("トンネルの状態確認に失敗しました: {e}"));
        }
    }
    let id = format!("tunnel-{}", NEXT_TUNNEL_ID.fetch_add(1, Ordering::SeqCst));
    let started = TunnelStarted {
        id: id.clone(),
        command: preview.clone(),
        summary: spec.summary.clone(),
    };
    tunnels().lock().map_err(|e| format!("トンネル管理のロックに失敗しました: {e}"))?.insert(
        id,
        ManagedTunnel {
            command: preview,
            summary: spec.summary,
            mode: spec.mode,
            child,
        },
    );
    Ok(started)
}

/// トンネルを停止する。存在しないIDは冪等に成功扱いとする。
pub fn stop_tunnel(id: &str) -> Result<(), String> {
    let mut guard = tunnels()
        .lock()
        .map_err(|e| format!("トンネル管理のロックに失敗しました: {e}"))?;
    if let Some(mut managed) = guard.remove(id) {
        let _ = managed.child.kill();
        let _ = managed.child.wait();
    }
    Ok(())
}

/// 起動中のトンネル一覧。終了済みはここで刈り取る。
pub fn list_tunnels() -> Vec<TunnelInfo> {
    let mut guard = match tunnels().lock() {
        Ok(guard) => guard,
        Err(_) => return Vec::new(),
    };
    let mut dead = Vec::new();
    for (id, managed) in guard.iter_mut() {
        match managed.child.try_wait() {
            Ok(Some(_)) => dead.push(id.clone()),
            Ok(None) => {}
            Err(_) => dead.push(id.clone()),
        }
    }
    for id in dead {
        guard.remove(&id);
    }
    let mut out: Vec<TunnelInfo> = guard
        .iter()
        .map(|(id, managed)| TunnelInfo {
            id: id.clone(),
            command: managed.command.clone(),
            summary: managed.summary.clone(),
            mode: managed.mode.clone(),
        })
        .collect();
    out.sort_by(|a, b| a.id.cmp(&b.id));
    out
}

/// アプリ終了時に全トンネルを停止する。
pub fn kill_all_tunnels() {
    let mut guard = match tunnels().lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    for (_, mut managed) in guard.drain() {
        let _ = managed.child.kill();
        let _ = managed.child.wait();
    }
}

// ---- `~/.ssh/config` の読み取り ----
// ホスト別名の選択候補用。読み取り専用で、パスは固定 (`$HOME/.ssh/config`)。
// ワイルドカード (`*`/`?`/`!`) を含むパターンは候補にしない。
// `Include` は辿らない (本体のみ)。

/// config内の既知ホスト1件。Frontend の `SshKnownHost` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct SshKnownHost {
    pub alias: String,
    pub hostname: Option<String>,
    pub user: Option<String>,
    pub port: Option<u16>,
}

/// `~/.ssh/config` の `Host` 別名一覧。ファイルが無ければ空を返す。
pub fn list_known_hosts() -> Result<Vec<SshKnownHost>, String> {
    let home = std::env::var("HOME").map_err(|e| format!("HOMEの取得に失敗しました: {e}"))?;
    let path = std::path::Path::new(&home).join(".ssh").join("config");
    let content = match std::fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => {
            return Err(format!(
                "SSH設定の読み取りに失敗しました ({}): {e}",
                path.display()
            ))
        }
    };
    Ok(parse_ssh_config(&content))
}

/// `ssh_config` 形式の `Host` ブロックをパースする。
/// `Host a b` の複数別名は別件として展開し、`HostName`/`User`/`Port` を拾う (先勝ち)。
/// `Match` 以降は条件付きなので属性を拾わない。
fn parse_ssh_config(content: &str) -> Vec<SshKnownHost> {
    let mut out: Vec<SshKnownHost> = Vec::new();
    let mut current: Vec<usize> = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let mut parts = trimmed.split_whitespace();
        let Some(keyword) = parts.next() else {
            continue;
        };
        let rest: Vec<&str> = parts.collect();
        if keyword.eq_ignore_ascii_case("host") {
            current.clear();
            for pattern in rest {
                if pattern.contains('*') || pattern.contains('?') || pattern.contains('!') {
                    continue;
                }
                if let Some(entry) = out.iter().find(|e| e.alias == *pattern) {
                    let index = out.iter().position(|e| e.alias == entry.alias).unwrap();
                    if !current.contains(&index) {
                        current.push(index);
                    }
                } else {
                    out.push(SshKnownHost {
                        alias: pattern.to_string(),
                        hostname: None,
                        user: None,
                        port: None,
                    });
                    current.push(out.len() - 1);
                }
            }
        } else if keyword.eq_ignore_ascii_case("match") {
            current.clear();
        } else if !current.is_empty() {
            if keyword.eq_ignore_ascii_case("hostname") {
                let value = rest.first().map(|v| v.to_string());
                for &index in &current {
                    if out[index].hostname.is_none() {
                        out[index].hostname = value.clone();
                    }
                }
            } else if keyword.eq_ignore_ascii_case("user") {
                let value = rest.first().map(|v| v.to_string());
                for &index in &current {
                    if out[index].user.is_none() {
                        out[index].user = value.clone();
                    }
                }
            } else if keyword.eq_ignore_ascii_case("port") {
                let value = rest.first().and_then(|v| v.parse::<u16>().ok());
                for &index in &current {
                    if out[index].port.is_none() {
                        out[index].port = value;
                    }
                }
            }
        }
    }
    out
}

fn validate_tunnel_mode(mode: &str) -> Result<(), String> {
    if matches!(mode, TUNNEL_LOCAL | TUNNEL_REMOTE | TUNNEL_DYNAMIC) {
        Ok(())
    } else {
        Err(format!("トンネルの方向が不正です: {mode}"))
    }
}

/// バインド先はループバックのみ。空欄は 127.0.0.1 に正規化する。
fn normalize_bind(bind: &str) -> Result<String, String> {
    if !ALLOWED_BINDS.contains(&bind) {
        return Err(format!(
            "バインド先は 127.0.0.1 / localhost のみです (外部公開は不可): {bind}"
        ));
    }
    if bind.is_empty() || bind == "localhost" {
        Ok("127.0.0.1".to_string())
    } else {
        Ok(bind.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_config, Ssh, SshQuery, MAX_TIMEOUT, MIN_TIMEOUT};

    const SAMPLE_G: &str = "hostname example.com\n\
        user alice\n\
        port 2222\n\
        identityfile ~/.ssh/id_ed25519\n\
        stricthostkeychecking accept-new\n\
        proxyjump none\n";

    fn base_query() -> SshQuery {
        SshQuery {
            host: "example.com".to_string(),
            user: String::new(),
            port: None,
            mode: "config".to_string(),
            connect_timeout: 10,
            strict: "accept-new".to_string(),
            verbose: false,
            transport: "auto".to_string(),
        }
    }

    #[test]
    fn previews_config_lookup() {
        let ssh = Ssh::new(base_query()).unwrap();
        assert_eq!(ssh.preview(), "ssh -T -G example.com");
    }

    #[test]
    fn previews_full_options() {
        let mut query = base_query();
        query.host = "example.com".to_string();
        query.user = "alice".to_string();
        query.port = Some(2222);
        query.verbose = true;
        query.transport = "4".to_string();
        let ssh = Ssh::new(query).unwrap();
        assert_eq!(ssh.preview(), "ssh -4 -v -T -p 2222 -G alice@example.com");
    }

    #[test]
    fn previews_connectivity_test() {
        let mut query = base_query();
        query.mode = "test".to_string();
        query.host = "example.com".to_string();
        query.user = "alice".to_string();
        query.port = Some(2222);
        query.connect_timeout = 15;
        query.strict = "ask".to_string();
        let ssh = Ssh::new(query).unwrap();
        assert_eq!(
            ssh.preview(),
            "ssh -T -o BatchMode=yes -o ConnectTimeout=15 -o StrictHostKeyChecking=ask -p 2222 alice@example.com exit"
        );
    }

    #[test]
    fn rejects_option_like_input() {
        let mut query = base_query();
        query.host = "-G".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.host = "example.com; rm -rf /".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.host = "alice@example.com".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.host = String::new();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.user = "-l".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.user = "alice bob".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.mode = "shell".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.mode = "test".to_string();
        query.strict = "no".to_string();
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.mode = "test".to_string();
        query.connect_timeout = MIN_TIMEOUT - 1;
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.mode = "test".to_string();
        query.connect_timeout = MAX_TIMEOUT + 1;
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.mode = "test".to_string();
        query.port = Some(0);
        assert!(Ssh::new(query).is_err());

        let mut query = base_query();
        query.transport = "x".to_string();
        assert!(Ssh::new(query).is_err());
    }

    #[test]
    fn parses_config_entries() {
        let entries = parse_config(SAMPLE_G);
        assert_eq!(entries.len(), 6);
        assert_eq!(entries[0].key, "hostname");
        assert_eq!(entries[0].value, "example.com");
        assert_eq!(entries[2].key, "port");
        assert_eq!(entries[2].value, "2222");
        assert_eq!(entries[3].value, "~/.ssh/id_ed25519");
    }

    #[test]
    fn tolerates_comments_and_blanks() {
        let entries = parse_config("# comment\n\nhostname example.com\n");
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].key, "hostname");
    }

    fn base_tunnel() -> super::TunnelQuery {
        super::TunnelQuery {
            mode: "local".to_string(),
            local_host: String::new(),
            local_port: 18080,
            remote_host: "internal.example.com".to_string(),
            remote_port: Some(80),
            host: "gateway.example.com".to_string(),
            user: "alice".to_string(),
            port: None,
            keepalive: true,
            transport: "auto".to_string(),
        }
    }

    #[test]
    fn previews_local_tunnel() {
        let spec = super::TunnelSpec::new(&base_tunnel()).unwrap();
        assert_eq!(
            spec.preview(),
            "ssh -N -T -o BatchMode=yes -o ExitOnForwardFailure=yes -o ServerAliveInterval=30 -o ServerAliveCountMax=3 -L 127.0.0.1:18080:internal.example.com:80 alice@gateway.example.com"
        );
    }

    #[test]
    fn previews_dynamic_tunnel() {
        let mut query = base_tunnel();
        query.mode = "dynamic".to_string();
        query.local_port = 11080;
        let spec = super::TunnelSpec::new(&query).unwrap();
        assert_eq!(
            spec.preview(),
            "ssh -N -T -o BatchMode=yes -o ExitOnForwardFailure=yes -o ServerAliveInterval=30 -o ServerAliveCountMax=3 -D 127.0.0.1:11080 alice@gateway.example.com"
        );
    }

    #[test]
    fn rejects_unsafe_tunnel_input() {
        let mut query = base_tunnel();
        query.local_host = "0.0.0.0".to_string();
        assert!(super::TunnelSpec::new(&query).is_err());

        let mut query = base_tunnel();
        query.mode = "shell".to_string();
        assert!(super::TunnelSpec::new(&query).is_err());

        let mut query = base_tunnel();
        query.remote_host = String::new();
        assert!(super::TunnelSpec::new(&query).is_err());

        let mut query = base_tunnel();
        query.remote_port = None;
        assert!(super::TunnelSpec::new(&query).is_err());

        let mut query = base_tunnel();
        query.local_port = 0;
        assert!(super::TunnelSpec::new(&query).is_err());

        let mut query = base_tunnel();
        query.host = "-L".to_string();
        assert!(super::TunnelSpec::new(&query).is_err());
    }

    #[test]
    fn reports_bind_failure() {
        // 特権ポートへのバインドは即失敗する (ネットワーク不要)
        let mut query = base_tunnel();
        query.local_port = 1;
        let err = super::start_tunnel(query).unwrap_err();
        assert!(err.contains("起動に失敗"), "unexpected: {err}");
    }

    #[test]
    fn stop_unknown_id_is_ok() {
        assert!(super::stop_tunnel("tunnel-does-not-exist").is_ok());
    }

    #[test]
    fn parses_known_hosts() {
        let content = "# sample\n\
            Host dev staging\n\
              HostName 192.168.1.10\n\
              User alice\n\
              Port 2222\n\
            \n\
            Host *.example.com\n\
              User bob\n\
            \n\
            Host prod\n\
              HostName prod.example.com\n\
            \n\
            Match host prod\n\
              User carol\n";
        let hosts = super::parse_ssh_config(content);
        assert_eq!(hosts.len(), 3);
        assert_eq!(hosts[0].alias, "dev");
        assert_eq!(hosts[0].hostname.as_deref(), Some("192.168.1.10"));
        assert_eq!(hosts[0].user.as_deref(), Some("alice"));
        assert_eq!(hosts[0].port, Some(2222));
        assert_eq!(hosts[1].alias, "staging");
        assert_eq!(hosts[1].hostname.as_deref(), Some("192.168.1.10"));
        assert_eq!(hosts[2].alias, "prod");
        assert_eq!(hosts[2].hostname.as_deref(), Some("prod.example.com"));
        // Match 配下の User は拾わない
        assert_eq!(hosts[2].user, None);
    }

    #[test]
    fn ignores_wildcard_hosts() {
        let hosts = super::parse_ssh_config("Host *\n  User bob\n");
        assert!(hosts.is_empty());
    }
}
