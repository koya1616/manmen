//! `pmset -a disablesleep` の定義と状態取得。
//! `AdminCommand` 実装の見本でもある。

use serde::Serialize;

use crate::spec::AdminCommand;
use crate::types::CommandResult;

#[derive(Debug, Clone, Serialize)]
pub struct DisablesleepState {
    /// 現在値 (取得できた場合のみ Some)
    pub enabled: Option<bool>,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

/// `sudo pmset -a disablesleep 1|0`
pub struct Disablesleep(pub bool);

impl AdminCommand for Disablesleep {
    const ID: &'static str = "pmset.disablesleep";
    const PROGRAM: &'static str = "/usr/bin/pmset";

    fn args(&self) -> Vec<String> {
        vec![
            "-a".to_string(),
            "disablesleep".to_string(),
            if self.0 { "1".to_string() } else { "0".to_string() },
        ]
    }

    fn preview(&self) -> String {
        // 表示用は短いコマンド名にする (実行時は PROGRAM の絶対パスを使う)
        format!(
            "sudo pmset -a disablesleep {}",
            if self.0 { "1" } else { "0" }
        )
    }
}

pub fn apply(enabled: bool) -> Result<CommandResult, String> {
    #[cfg(not(target_os = "macos"))]
    return Err("pmset is only supported on macOS".to_string());

    #[cfg(target_os = "macos")]
    return Disablesleep(enabled).run();
}

/// 現在値を取得する (管理者権限不要: `pmset -g`)
pub fn current() -> Result<DisablesleepState, String> {
    #[cfg(not(target_os = "macos"))]
    return Err("pmset is only supported on macOS".to_string());

    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new(Disablesleep::PROGRAM)
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
            enabled: parse_state(&stdout),
            stdout,
            stderr,
            exit_code,
        })
    }
}

/// `pmset -g` の出力から disablesleep / SleepDisabled を探す
fn parse_state(output: &str) -> Option<bool> {
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
    use super::{parse_state, Disablesleep};
    use crate::spec::AdminCommand;

    #[test]
    fn previews_exact_initial_commands() {
        assert_eq!(
            Disablesleep(true).preview(),
            "sudo pmset -a disablesleep 1"
        );
        assert_eq!(
            Disablesleep(false).preview(),
            "sudo pmset -a disablesleep 0"
        );
    }

    #[test]
    fn parses_pmset_g_output() {
        assert_eq!(
            parse_state("displaysleep         10\ndisablesleep         1\n"),
            Some(true)
        );
        assert_eq!(parse_state("SleepDisabled         0\n"), Some(false));
        assert_eq!(parse_state("displaysleep 10\n"), None);
    }
}
