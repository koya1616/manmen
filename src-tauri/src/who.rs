//! `who` の定義・実行・出力パース。管理者権限は不要。
//! 引数なし。`who` の各行は `USER TTY MON DD TIME` なので
//! 空白区切りで割り、3つ目以降をログイン日時として束ねる。

use serde::Serialize;

use crate::privileged;

/// ログイン中ユーザの1行。Frontend の `WhoEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WhoEntry {
    pub user: String,
    pub tty: String,
    pub login: String,
}

/// `get_who` の返却値。Frontend の `WhoSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct WhoSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub users: Vec<WhoEntry>,
    pub count: usize,
    pub stderr: String,
}

/// `who` (引数なし)
pub fn current() -> Result<WhoSnapshot, String> {
    let preview = "who".to_string();
    let output = privileged::execute_plain("/usr/bin/who", &[], preview.clone())?;
    let users = parse_entries(&output.stdout);
    Ok(WhoSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        count: users.len(),
        users,
        stderr: output.stderr.trim().to_string(),
    })
}

/// `USER TTY MON DD TIME` をパースする。トークン不足の行は捨てる。
fn parse_entries(stdout: &str) -> Vec<WhoEntry> {
    stdout.lines().filter_map(parse_line).collect()
}

fn parse_line(line: &str) -> Option<WhoEntry> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 5 {
        return None;
    }
    Some(WhoEntry {
        user: tokens[0].to_string(),
        tty: tokens[1].to_string(),
        login: tokens[2..].join(" "),
    })
}

#[cfg(test)]
mod tests {
    use super::{current, parse_line};

    #[test]
    fn parses_who_line() {
        let entry = parse_line("aoyamakoya       console      Sep 30 12:56 ").unwrap();
        assert_eq!(entry.user, "aoyamakoya");
        assert_eq!(entry.tty, "console");
        assert_eq!(entry.login, "Sep 30 12:56");
    }

    #[test]
    fn parses_padded_day() {
        // 日が1桁のときは空白埋めされるが split_whitespace で吸収できる
        let entry = parse_line("aoyamakoya       ttys006      Oct  3 10:14 ").unwrap();
        assert_eq!(entry.login, "Oct 3 10:14");
    }

    #[test]
    fn drops_short_lines() {
        assert!(parse_line("").is_none());
        assert!(parse_line("aoyamakoya console").is_none());
    }

    #[test]
    fn runs_who() {
        let result = current().expect("who");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.command, "who");
        assert_eq!(result.count, result.users.len());
    }
}
