use serde::Serialize;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

const PMSET: &str = "/usr/bin/pmset";
const SUDO: &str = "/usr/bin/sudo";
/// sudoタイムスタンプの更新間隔 (macOSデフォルトの有効期限は約5分)
const KEEPALIVE_INTERVAL_SECS: u64 = 120;
const KEEPALIVE_MAX_FAILURES: u32 = 3;

/// パスワード省略が有効かどうか (アプリ起動中のみ有効・デフォルトON)
static REMEMBER: AtomicBool = AtomicBool::new(true);
static KEEPALIVE_ACTIVE: AtomicBool = AtomicBool::new(false);

#[derive(Debug, Clone, Serialize)]
pub struct DisablesleepState {
    /// 現在値 (取得できた場合のみ Some)
    pub enabled: Option<bool>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DisablesleepResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    /// 実行した内容のプレビュー (表示用)
    pub command: String,
}

pub fn preview_command(enabled: bool) -> String {
    format!(
        "sudo pmset -a disablesleep {}",
        if enabled { 1 } else { 0 }
    )
}

/// パスワード省略のON/OFF (OFFにしても既存のsudoタイムスタンプは消さない)
#[tauri::command]
pub fn set_remember(enabled: bool) {
    REMEMBER.store(enabled, Ordering::SeqCst);
}

/// 現在の disablesleep 値を取得する (管理者権限不要: `pmset -g`)
#[tauri::command]
pub fn get_disablesleep() -> Result<DisablesleepState, String> {
    #[cfg(not(target_os = "macos"))]
    return Err("get_disablesleep is only supported on macOS".to_string());

    #[cfg(target_os = "macos")]
    {
        let output = Command::new(PMSET)
            .arg("-g")
            .output()
            .map_err(|e| format!("failed to run pmset -g: {e}"))?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        if !output.status.success() {
            return Err(format!("pmset -g failed (exit {exit_code}): {stderr}"));
        }

        Ok(DisablesleepState {
            enabled: parse_disablesleep(&stdout),
            stdout,
            stderr,
            exit_code,
        })
    }
}

/// `sudo pmset -a disablesleep 1|0` を実行する。
/// 省略ON時: 初回のみGUIダイアログでパスワード入力→以降はsudoタイムスタンプで省略
/// (アプリ起動中はバックグラウンドでタイムスタンプを更新し続ける)。
/// 省略OFF時: 毎回 osascript の管理者認証ダイアログを出す。
#[tauri::command]
pub fn set_disablesleep(enabled: bool) -> Result<DisablesleepResult, String> {
    #[cfg(not(target_os = "macos"))]
    return Err("set_disablesleep is only supported on macOS".to_string());

    #[cfg(target_os = "macos")]
    {
        let value = if enabled { "1" } else { "0" };
        let command = preview_command(enabled);
        let pmset_args = ["-a", "disablesleep", value];

        if REMEMBER.load(Ordering::SeqCst) {
            // 1. タイムスタンプが有効ならパスワードなしで実行
            let output = Command::new(SUDO)
                .args(["-n", PMSET])
                .args(pmset_args)
                .output()
                .map_err(|e| format!("failed to run sudo: {e}"))?;
            if output.status.success() {
                start_keepalive();
                return Ok(collect(output, command, true));
            }
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if !needs_password(&stderr) {
                // sudo自体は通ったがpmsetが失敗 → 認証不要なのでそのまま返す
                return Ok(collect(output, command, false));
            }

            // 2. 初回のみGUIダイアログで認証 (sudoタイムスタンプが刻まれる)
            let askpass = askpass_path()?;
            let output = Command::new(SUDO)
                .args(["-A", PMSET])
                .args(pmset_args)
                .env("SUDO_ASKPASS", &askpass)
                .output()
                .map_err(|e| format!("failed to run sudo: {e}"))?;
            let success = output.status.success();
            if success {
                start_keepalive();
            }
            return Ok(collect(output, command, success));
        }

        // 省略OFF: osascript の管理者認証ダイアログ経由で実行
        // パスワードをアプリ側で扱わない。
        let shell = format!("{PMSET} -a disablesleep {value}");
        let output = Command::new("/usr/bin/osascript")
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
        let success = output.status.success();
        Ok(collect(output, command, success))
    }
}

fn collect(
    output: std::process::Output,
    command: String,
    success: bool,
) -> DisablesleepResult {
    DisablesleepResult {
        success,
        exit_code: output.status.code().unwrap_or(-1),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        command,
    }
}

/// sudoがパスワード要求で失敗したかどうか
fn needs_password(stderr: &str) -> bool {
    stderr.to_lowercase().contains("password")
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
            if !REMEMBER.load(Ordering::SeqCst) {
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

/// `pmset -g` の出力から disablesleep / SleepDisabled を探す
fn parse_disablesleep(output: &str) -> Option<bool> {
    for line in output.lines() {
        let lower = line.to_lowercase();
        // 例: "disablesleep        1" / "SleepDisabled         0"
        if lower.contains("disablesleep") || lower.contains("sleepdisabled") {
            if lower.split_whitespace().any(|t| t == "1") {
                return Some(true);
            }
            if lower.split_whitespace().any(|t| t == "0") {
                return Some(false);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{needs_password, parse_disablesleep, preview_command};

    #[test]
    fn previews_exact_initial_commands() {
        assert_eq!(preview_command(true), "sudo pmset -a disablesleep 1");
        assert_eq!(preview_command(false), "sudo pmset -a disablesleep 0");
    }

    #[test]
    fn parses_pmset_g_output() {
        assert_eq!(
            parse_disablesleep("displaysleep         10\ndisablesleep         1\n"),
            Some(true)
        );
        assert_eq!(parse_disablesleep("SleepDisabled         0\n"), Some(false));
        assert_eq!(parse_disablesleep("displaysleep 10\n"), None);
    }

    #[test]
    fn detects_password_required() {
        assert!(needs_password("sudo: a password is required"));
        assert!(needs_password("sudo: no password was provided"));
        assert!(!needs_password("pmset: unknown option"));
        assert!(!needs_password(""));
    }
}
