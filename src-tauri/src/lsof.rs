//! `lsof` の定義・実行・出力パース。管理者権限は不要。
//! 読み取り専用の絞り込みだけを許可する。結果が膨大になるため、
//! 絞り込み (PID・ユーザ・コマンド・ネットワーク) が1つも無い実行は受け付けない。
//!
//! 常に `-a` (AND条件)・`-P` (ポート名解決なし)・`-n` (ホスト名解決なし)・
//! `+c 0` (コマンド名の切り詰めなし) を付ける。NAME列は空白を含むため末尾に固定。

use serde::{Deserialize, Serialize};

use crate::common::parse::parse_table;
use crate::common::validate::{parse_pids as parse_pids_common, validate_user};
use crate::privileged;

/// `-i` のプロトコル。`man lsof` の `[46][protocol]` のうち日常的なものに限る。
pub const ALLOWED_PROTOCOLS: &[&str] = &["any", "TCP", "UDP"];
/// `-s` のTCP状態。日常的なものに限る。
pub const ALLOWED_STATES: &[&str] = &["any", "LISTEN", "ESTABLISHED"];

/// 固定の表示列。Frontend の表示と一致させること。
pub const COLUMNS: &[&str] =
    &["command", "pid", "user", "fd", "type", "device", "size", "node", "name"];
const MAX_PIDS: usize = 16;

/// Frontend の `getLsof` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LsofQuery {
    #[serde(default)]
    pub pids: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub comm: String,
    #[serde(default = "default_protocol")]
    pub protocol: String,
    #[serde(default)]
    pub port: String,
    #[serde(default)]
    pub host: String,
    #[serde(default = "default_state")]
    pub state: String,
}

fn default_protocol() -> String {
    "any".to_string()
}

fn default_state() -> String {
    "any".to_string()
}

/// `get_lsof` の返却値。Frontend の `LsofSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct LsofSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub count: usize,
    pub stderr: String,
}

/// `lsof -a -P -n +c 0` に、検証済みの絞り込みだけを足したもの。
pub struct Lsof {
    args: Vec<String>,
}

impl Lsof {
    pub fn new(query: LsofQuery) -> Result<Self, String> {
        Ok(Self {
            args: build_args(query)?,
        })
    }

    pub fn preview(&self) -> String {
        format!("lsof {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<LsofSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("lsof", &refs, preview.clone())?;
        let rows = parse_rows(&output.stdout);
        Ok(LsofSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            columns: COLUMNS.iter().map(|c| c.to_string()).collect(),
            rows: rows.clone(),
            count: rows.len(),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: LsofQuery) -> Result<LsofSnapshot, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("lsof is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return Lsof::new(query).and_then(|cmd| cmd.run());
}

fn build_args(query: LsofQuery) -> Result<Vec<String>, String> {
    let user = query.user.trim();
    validate_user(user)?;
    let comm = query.comm.trim();
    validate_comm(comm)?;
    let pids = parse_pids(&query.pids)?;
    let protocol = query.protocol.trim().to_uppercase();
    // "any" は小文字のまま来るため正規化する
    let protocol = if protocol == "ANY" { "any".to_string() } else { protocol };
    validate_protocol(&protocol)?;
    let port = query.port.trim();
    validate_port(port)?;
    let host = query.host.trim();
    validate_host(host)?;
    let state = query.state.trim().to_uppercase();
    let state = if state == "ANY" { "any".to_string() } else { state };
    validate_state(&state)?;
    if protocol == "UDP" && state != "any" {
        return Err("状態の絞り込みは TCP のときのみ指定できます".to_string());
    }

    let net_active =
        protocol != "any" || !port.is_empty() || !host.is_empty() || state != "any";
    if pids.is_empty() && user.is_empty() && comm.is_empty() && !net_active {
        return Err("絞り込み (PID・ユーザ・コマンド・ネットワーク) を1つ以上指定してください".to_string());
    }

    let mut args = vec![
        "-a".to_string(),
        "-P".to_string(),
        "-n".to_string(),
        "+c".to_string(),
        "0".to_string(),
    ];
    if !pids.is_empty() {
        args.push("-p".to_string());
        args.push(pids.join(","));
    }
    if !user.is_empty() {
        args.push("-u".to_string());
        args.push(user.to_string());
    }
    if !comm.is_empty() {
        args.push("-c".to_string());
        args.push(comm.to_string());
    }
    if net_active {
        let mut spec = "-i".to_string();
        if protocol != "any" {
            spec.push_str(&protocol);
        }
        if !host.is_empty() {
            spec.push('@');
            spec.push_str(host);
        }
        if !port.is_empty() {
            spec.push(':');
            spec.push_str(port);
        }
        args.push(spec);
        if state != "any" {
            args.push(format!("-sTCP:{state}"));
        }
    }
    Ok(args)
}

fn validate_protocol(protocol: &str) -> Result<(), String> {
    if ALLOWED_PROTOCOLS.contains(&protocol) {
        Ok(())
    } else {
        Err(format!("プロトコルが不正です: {protocol}"))
    }
}

fn validate_state(state: &str) -> Result<(), String> {
    if ALLOWED_STATES.contains(&state) {
        Ok(())
    } else {
        Err(format!("状態が不正です: {state}"))
    }
}

/// `-c` はコマンド名の前方一致。オプション解釈を防ぐため英数字等に制限する。
fn validate_comm(comm: &str) -> Result<(), String> {
    if comm.is_empty() {
        return Ok(());
    }
    let ok = comm.len() <= 32
        && comm
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
        && !comm.starts_with('-');
    if ok {
        Ok(())
    } else {
        Err(format!("コマンド名が不正です: {comm}"))
    }
}

/// 単一ポート (`80`) または範囲 (`80-443`)。各1〜65535。
fn validate_port(port: &str) -> Result<(), String> {
    if port.is_empty() {
        return Ok(());
    }
    let mut parts = port.split('-');
    let first = parts.next().unwrap_or("");
    let second = parts.next();
    if parts.next().is_some() {
        return Err(format!("ポートが不正です: {port}"));
    }
    validate_port_num(first).map_err(|_| format!("ポートが不正です: {port}"))?;
    if let Some(second) = second {
        validate_port_num(second).map_err(|_| format!("ポートが不正です: {port}"))?;
    }
    Ok(())
}

fn validate_port_num(num: &str) -> Result<u32, String> {
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return Err(num.to_string());
    }
    match num.parse::<u32>() {
        Ok(n) if (1..=65535).contains(&n) => Ok(n),
        _ => Err(num.to_string()),
    }
}

fn validate_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Ok(());
    }
    if host.len() > 253 || host.starts_with('-') {
        return Err(format!("ホストが不正です: {host}"));
    }
    if host.parse::<std::net::IpAddr>().is_ok() {
        return Ok(());
    }
    if host.contains('@') || host.contains(' ') || host.contains('/') || host.contains(':') {
        return Err(format!("ホストが不正です: {host}"));
    }
    let ok = host
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if !ok || host.contains("..") {
        return Err(format!("ホストが不正です: {host}"));
    }
    Ok(())
}

fn parse_pids(raw: &str) -> Result<Vec<String>, String> {
    parse_pids_common(raw, MAX_PIDS)
}

/// 先頭の見出し行を除き、9列に切る。末尾の NAME 列は空白を含むため残り全部を1セルにする。
fn parse_rows(stdout: &str) -> Vec<Vec<String>> {
    parse_table(stdout, COLUMNS.len())
}

#[cfg(test)]
mod tests {
    use super::{parse_rows, Lsof, LsofQuery, MAX_PIDS};

    fn base_query() -> LsofQuery {
        LsofQuery {
            pids: String::new(),
            user: String::new(),
            comm: String::new(),
            protocol: "any".to_string(),
            port: String::new(),
            host: String::new(),
            state: "any".to_string(),
        }
    }

    #[test]
    fn previews_network_filter() {
        let mut query = base_query();
        query.protocol = "TCP".to_string();
        query.port = "80".to_string();
        query.state = "LISTEN".to_string();
        let lsof = Lsof::new(query).unwrap();
        assert_eq!(
            lsof.preview(),
            "lsof -a -P -n +c 0 -iTCP:80 -sTCP:LISTEN"
        );
    }

    #[test]
    fn previews_process_filters() {
        let mut query = base_query();
        query.pids = "1, 2,1".to_string();
        query.user = "root".to_string();
        query.comm = "launchd".to_string();
        let lsof = Lsof::new(query).unwrap();
        assert_eq!(
            lsof.preview(),
            "lsof -a -P -n +c 0 -p 1,2 -u root -c launchd"
        );
    }

    #[test]
    fn previews_udp_host() {
        let mut query = base_query();
        query.protocol = "UDP".to_string();
        query.host = "127.0.0.1".to_string();
        query.port = "53".to_string();
        let lsof = Lsof::new(query).unwrap();
        assert_eq!(lsof.preview(), "lsof -a -P -n +c 0 -iUDP@127.0.0.1:53");
    }

    #[test]
    fn rejects_unsafe_options() {
        // 絞り込みなしは受け付けない
        assert!(Lsof::new(base_query()).is_err());

        let mut query = base_query();
        query.protocol = "ICMP".to_string();
        query.port = "80".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.protocol = "UDP".to_string();
        query.state = "LISTEN".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.port = "abc".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.port = "0".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.port = "80-70-60".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.host = "example.com;id".to_string();
        query.port = "80".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.comm = "-c".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.user = "root;id".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.pids = "1,-1".to_string();
        assert!(Lsof::new(query).is_err());

        let mut query = base_query();
        query.pids = (0..=MAX_PIDS).map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        assert!(Lsof::new(query).is_err());
    }

    #[test]
    fn parses_rows_keeping_name_intact() {
        let sample = "COMMAND     PID       USER   FD   TYPE             DEVICE SIZE/OFF NODE NAME\n\
            circleci-yaml-language-server  2610 aoyamakoya    6u  IPv4 0xd640c1094841b869      0t0  TCP 127.0.0.1:58450 (LISTEN)\n\
            zsh     76677 aoyamakoya  cwd    DIR               1,16      640           138167382 /Users/aoyamakoya/My Docs/file name.txt\n";
        let rows = parse_rows(sample);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], "circleci-yaml-language-server");
        assert_eq!(rows[0][1], "2610");
        // ネットワーク行は NODE 列にプロトコル (TCP) が入る
        assert_eq!(rows[0][7], "TCP");
        assert_eq!(rows[0][8], "127.0.0.1:58450 (LISTEN)");
        assert_eq!(rows[1][8], "/Users/aoyamakoya/My Docs/file name.txt");
    }

    #[test]
    fn tolerates_empty_output() {
        let rows = parse_rows("COMMAND     PID USER FD TYPE DEVICE SIZE/OFF NODE NAME\n");
        assert!(rows.is_empty());
    }
}
