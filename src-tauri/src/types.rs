use serde::Serialize;

/// 全コマンド共通の実行結果。
/// Frontend の `CommandResult` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct CommandResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    /// 実行した内容のプレビュー (表示用)
    pub command: String,
}

impl CommandResult {
    pub fn from_output(output: std::process::Output, command: String) -> Self {
        Self {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            command,
        }
    }
}
