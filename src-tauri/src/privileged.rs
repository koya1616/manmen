//! 管理者権限実行の共通基盤。
//! コマンド固有の知識は持たせないこと。新しいコマンドもこのモジュール経由で実行する。

use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::types::CommandResult;

const SUDO: &str = "/usr/bin/sudo";
const OSASCRIPT: &str = "/usr/bin/osascript";
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

/// 管理者権限で実行する (省略ONの経路: `sudo -n` → 初回のみ `sudo -A` でGUI認証)。
/// 成功時はsudoタイムスタンプの維持スレッドを起動する。
pub fn execute_privileged(
    program: &str,
    args: &[&str],
    preview: String,
) -> Result<CommandResult, String> {
    // 1. タイムスタンプが有効ならパスワードなしで実行
    let output = Command::new(SUDO)
        .arg("-n")
        .arg(program)
        .args(args)
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
        .arg(program)
        .args(args)
        .env("SUDO_ASKPASS", &askpass)
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
    let shell = std::iter::once(program)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ");
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
    use super::needs_password;

    #[test]
    fn detects_password_required() {
        assert!(needs_password(b"sudo: a password is required"));
        assert!(needs_password(b"sudo: no password was provided"));
        assert!(!needs_password(b"pmset: unknown option"));
        assert!(!needs_password(b""));
    }
}
