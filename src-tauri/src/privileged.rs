//! 管理者権限実行の共通基盤。
//! コマンド固有の知識は持たせないこと。新しいコマンドもこのモジュール経由で実行する。

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::types::CommandResult;

const SUDO: &str = "/usr/bin/sudo";
const OSASCRIPT: &str = "/usr/bin/osascript";
/// Finder/Dock 起動のバンドルアプリは最小 PATH
/// (`/usr/bin:/bin:/usr/sbin:/sbin` 程度) で起動し、Homebrew や
/// Docker Desktop の CLI 位置を見に行かない。`tauri dev` は
/// ターミナルの PATH を継承するためだけに動いていた。
/// bare なプログラム名はここで絶対パスに解決する。
const FALLBACK_DIRS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/Applications/Docker.app/Contents/Resources/bin",
    "/usr/bin",
    "/bin",
    "/usr/sbin",
    "/sbin",
];
/// sudoタイムスタンプの更新間隔 (macOSデフォルトの有効期限は約5分)
const KEEPALIVE_INTERVAL_SECS: u64 = 120;
const KEEPALIVE_MAX_FAILURES: u32 = 3;

/// パスワード省略が有効かどうか (アプリ起動中のみ有効・デフォルトON)
static REMEMBER: AtomicBool = AtomicBool::new(true);
static KEEPALIVE_ACTIVE: AtomicBool = AtomicBool::new(false);

pub fn set_remember(enabled: bool) {
    REMEMBER.store(enabled, Ordering::SeqCst);
}

pub fn remember_enabled() -> bool {
    REMEMBER.load(Ordering::SeqCst)
}

/// 権限不要コマンドをそのまま実行する (管理者権限を使わない)。
pub fn execute_plain(
    program: &str,
    args: &[&str],
    preview: String,
) -> Result<CommandResult, String> {
    let resolved = resolve_program(program);
    let output = std::process::Command::new(&resolved)
        .args(args)
        .env("PATH", expanded_path())
        .output()
        .map_err(|e| spawn_error(program, &resolved, e))?;
    Ok(CommandResult::from_output(output, preview))
}

/// 管理者権限で実行する (省略ONの経路: `sudo -n` → 初回のみ `sudo -A` でGUI認証)。
/// 成功時はsudoタイムスタンプの維持スレッドを起動する。
pub fn execute_privileged(
    program: &str,
    args: &[&str],
    preview: String,
) -> Result<CommandResult, String> {
    // sudo は secure_path を使うため PATH 引き継ぎに頼らず絶対パスで渡す。
    let resolved = resolve_program(program);
    // 1. タイムスタンプが有効ならパスワードなしで実行
    let output = Command::new(SUDO)
        .arg("-n")
        .arg(&resolved)
        .args(args)
        .env("PATH", expanded_path())
        .output()
        .map_err(|e| format!("failed to run sudo: {e}"))?;
    if output.status.success() {
        start_keepalive();
        return Ok(CommandResult::from_output(output, preview));
    }
    if !needs_password(&output.stderr) {
        // sudo自体は通ったがコマンドが失敗 → 認証不要なのでそのまま返す
        return Ok(CommandResult::from_output(output, preview));
    }

    // 2. 初回のみGUIダイアログで認証 (sudoタイムスタンプが刻まれる)
    let askpass = askpass_path()?;
    let output = Command::new(SUDO)
        .arg("-A")
        .arg(&resolved)
        .args(args)
        .env("SUDO_ASKPASS", &askpass)
        .env("PATH", expanded_path())
        .output()
        .map_err(|e| format!("failed to run sudo: {e}"))?;
    if output.status.success() {
        start_keepalive();
    }
    Ok(CommandResult::from_output(output, preview))
}

/// 管理者権限で実行する (省略OFFの経路: 毎回 osascript の認証ダイアログ)。
/// パスワードをアプリ側で扱わない。
pub fn execute_with_prompt(
    program: &str,
    args: &[&str],
    preview: String,
) -> Result<CommandResult, String> {
    // `do shell script` は /bin/sh の最小 PATH で動くため絶対パス + 引用が必須。
    let resolved = resolve_program(program);
    let mut parts = vec![shell_escape(&resolved)];
    parts.extend(args.iter().map(|a| shell_escape(a)));
    let shell = parts.join(" ");
    let output = Command::new(OSASCRIPT)
        .args([
            "-e",
            "on run argv",
            "-e",
            "do shell script (item 1 of argv) with administrator privileges",
            "-e",
            "end run",
            &shell,
        ])
        .output()
        .map_err(|e| format!("failed to run osascript: {e}"))?;
    Ok(CommandResult::from_output(output, preview))
}

/// bare なプログラム名を絶対パスに解決する。`/` を含むものはそのまま返す。
/// 既存 PATH → フォールバック定数 → `$HOME/.docker/bin` の順に探す。
fn resolve_program(program: &str) -> String {
    if program.contains('/') {
        return program.to_string();
    }
    for dir in search_dirs() {
        let candidate = std::path::Path::new(&dir).join(program);
        if candidate.is_file() {
            return candidate.to_string_lossy().into_owned();
        }
    }
    program.to_string()
}

/// 子プロセス用の PATH。不足しているフォールバックだけを先頭に足す。
fn expanded_path() -> String {
    let current = std::env::var("PATH").unwrap_or_default();
    join_path(&current, &search_dirs())
}

fn search_dirs() -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();
    if let Ok(path) = std::env::var("PATH") {
        for part in path.split(':') {
            if !part.is_empty() && !dirs.iter().any(|d| d == part) {
                dirs.push(part.to_string());
            }
        }
    }
    for dir in FALLBACK_DIRS {
        if !dirs.iter().any(|d| d == dir) {
            dirs.push(dir.to_string());
        }
    }
    if let Ok(home) = std::env::var("HOME") {
        let docker_bin = format!("{home}/.docker/bin");
        if !dirs.iter().any(|d| d == &docker_bin) {
            dirs.push(docker_bin);
        }
    }
    dirs
}

fn join_path(current: &str, dirs: &[String]) -> String {
    if current.is_empty() {
        return dirs.join(":");
    }
    let mut out: Vec<&str> = current.split(':').collect();
    for dir in dirs {
        if !out.iter().any(|d| *d == dir) {
            out.push(dir);
        }
    }
    out.join(":")
}

/// `sh` に安全に渡せるよう単一引用符で囲む。
fn shell_escape(arg: &str) -> String {
    if !arg.is_empty()
        && arg
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '/' | ':' | '@' | '+' | ',' | '=' | '%'))
    {
        return arg.to_string();
    }
    format!("'{}'", arg.replace('\'', "'\\''"))
}

/// spawn 失敗時は解決先と PATH を含めて返す (Finder 起動時の ENOENT 調査用)。
fn spawn_error(program: &str, resolved: &str, e: std::io::Error) -> String {
    let path = std::env::var("PATH").unwrap_or_default();
    if resolved != program {
        return format!("failed to run {program} ({resolved}, PATH={path}): {e}");
    }
    if e.kind() == std::io::ErrorKind::NotFound {
        return format!(
            "failed to run {program} (PATH={path}, searched={}): {e}",
            search_dirs().join(":")
        );
    }
    format!("failed to run {program}: {e}")
}

/// sudoがパスワード要求で失敗したかどうか
fn needs_password(stderr: &[u8]) -> bool {
    String::from_utf8_lossy(stderr).to_lowercase().contains("password")
}

/// sudo -A 用の askpass ヘルパー。
/// パスワード入力ダイアログを表示し、入力値をstdoutに返すだけのスクリプト。
/// パスワードはヘルパー→sudoに直接渡り、アプリ側は一切扱わない。
fn askpass_path() -> Result<std::path::PathBuf, String> {
    let path = std::env::temp_dir().join("manmen-askpass.sh");
    if !path.is_file() {
        std::fs::write(
            &path,
            r#"#!/bin/sh
exec /usr/bin/osascript -e 'display dialog "Manmen: 管理者パスワードを入力してください" default answer "" with hidden answer with title "Manmen"' -e 'text returned of result'
"#,
        )
        .map_err(|e| format!("failed to write askpass helper: {e}"))?;
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700))
            .map_err(|e| format!("failed to chmod askpass helper: {e}"))?;
    }
    Ok(path)
}

/// sudoタイムスタンプをアプリ起動中に維持する。
/// パスワードの保存はしない (`sudo -n -v` は有効なタイムスタンプの更新のみ)。
fn start_keepalive() {
    if KEEPALIVE_ACTIVE
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_err()
    {
        return;
    }
    std::thread::spawn(|| {
        let mut failures = 0;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(KEEPALIVE_INTERVAL_SECS));
            if !remember_enabled() {
                break;
            }
            match Command::new(SUDO).args(["-n", "-v"]).output() {
                Ok(output) if output.status.success() => failures = 0,
                _ => {
                    failures += 1;
                    if failures >= KEEPALIVE_MAX_FAILURES {
                        break;
                    }
                }
            }
        }
        KEEPALIVE_ACTIVE.store(false, Ordering::SeqCst);
    });
}

#[cfg(test)]
mod tests {
    use super::{join_path, needs_password, resolve_program, search_dirs, shell_escape};

    #[test]
    fn detects_password_required() {
        assert!(needs_password(b"sudo: a password is required"));
        assert!(needs_password(b"sudo: no password was provided"));
        assert!(!needs_password(b"pmset: unknown option"));
        assert!(!needs_password(b""));
    }

    #[test]
    fn keeps_absolute_path_as_is() {
        assert_eq!(resolve_program("/usr/bin/pmset"), "/usr/bin/pmset");
        assert_eq!(
            resolve_program("/opt/homebrew/bin/docker"),
            "/opt/homebrew/bin/docker"
        );
    }

    #[test]
    fn search_dirs_include_gui_fallbacks() {
        let dirs = search_dirs().join(":");
        assert!(dirs.contains("/opt/homebrew/bin"));
        assert!(dirs.contains("/usr/local/bin"));
        assert!(dirs.contains("/Applications/Docker.app/Contents/Resources/bin"));
    }

    #[test]
    fn resolves_existing_binary_via_path() {
        // `sh` は通常どの PATH でも見つかるはず
        let resolved = resolve_program("sh");
        assert!(
            resolved.ends_with("/sh"),
            "sh should resolve to absolute path, got {resolved}"
        );
    }

    #[test]
    fn keeps_missing_binary_for_caller_error() {
        let name = "manmen-definitely-missing-binary-xyz";
        assert_eq!(resolve_program(name), name);
    }

    #[test]
    fn join_path_appends_missing_only() {
        let joined = join_path("/usr/bin:/bin", &["/usr/bin".to_string(), "/opt/homebrew/bin".to_string()]);
        assert_eq!(joined, "/usr/bin:/bin:/opt/homebrew/bin");
    }

    #[test]
    fn escapes_shell_args() {
        assert_eq!(shell_escape("/usr/bin/pmset"), "/usr/bin/pmset");
        assert_eq!(shell_escape("-a"), "-a");
        assert_eq!(shell_escape("a b"), "'a b'");
        assert_eq!(shell_escape("a'b"), "'a'\\''b'");
        assert_eq!(
            shell_escape("/Applications/Docker.app/Contents/Resources/bin/docker"),
            "/Applications/Docker.app/Contents/Resources/bin/docker"
        );
        assert_eq!(
            shell_escape("/Applications/Docker Desktop.app/bin/docker"),
            "'/Applications/Docker Desktop.app/bin/docker'"
        );
    }
}
