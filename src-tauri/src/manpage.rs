//! `man <topic>` の定義と実行。管理者権限は不要。

use crate::privileged;
use crate::types::CommandResult;

/// `man <topic>`
pub struct Manpage {
    pub topic: String,
}

impl Manpage {
    pub fn new(topic: &str) -> Result<Self, String> {
        let topic = topic.trim().to_string();
        validate_topic(&topic)?;
        Ok(Self { topic })
    }

    pub fn preview(&self) -> String {
        format!("man {}", self.topic)
    }

    pub fn run(&self) -> Result<CommandResult, String> {
        let preview = self.preview();
        let mut result =
            privileged::execute_plain("/usr/bin/man", &[&self.topic], preview)?;
        // man はパイプ出力時に強調用の overstrike を含むため除去する
        result.stdout = strip_overstrike(&result.stdout);
        Ok(result)
    }
}

/// man 自体のオプション解釈を防ぐため、トピック名を制限する。
/// 先頭 `-` 禁止・英数字と `_-.+` のみ許可。
fn validate_topic(topic: &str) -> Result<(), String> {
    if topic.is_empty() {
        return Err("トピック名を入力してください".to_string());
    }
    let valid = !topic.starts_with('-')
        && topic
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.' | '+'));
    if !valid {
        return Err(format!("トピック名が不正です: {topic}"));
    }
    Ok(())
}

/// overstrike (太字 `X\x08X`・下線 `_\x08X`) を除去する (`col -b` 相当)。
fn strip_overstrike(output: &str) -> String {
    let mut cleaned = String::with_capacity(output.len());
    for ch in output.chars() {
        if ch == '\u{8}' {
            cleaned.pop();
        } else {
            cleaned.push(ch);
        }
    }
    cleaned
}

#[cfg(test)]
mod tests {
    use super::{strip_overstrike, validate_topic, Manpage};

    #[test]
    fn previews_man_command() {
        assert_eq!(Manpage::new("pmset").unwrap().preview(), "man pmset");
    }

    #[test]
    fn rejects_option_like_topics() {
        assert!(Manpage::new("-l").is_err());
        assert!(Manpage::new("").is_err());
        assert!(Manpage::new("   ").is_err());
        assert!(Manpage::new("a;rm -rf /").is_err());
        assert!(Manpage::new("pmset").is_ok());
        assert!(Manpage::new("caffeinate").is_ok());
        assert!(validate_topic("launchctl").is_ok());
    }

    #[test]
    fn strips_overstrike() {
        // 太字: X\x08X → X / 下線: _\x08X → X
        assert_eq!(strip_overstrike("a\x08ab\x08bc"), "abc");
        assert_eq!(strip_overstrike("_\x08ab"), "ab");
        assert_eq!(strip_overstrike("plain text"), "plain text");
    }
}
