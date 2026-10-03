//! `w` の定義・実行・出力パース。管理者権限は不要。
//! 引数なし。1行目は稼働・負荷ヘッダ、2行目は列名、3行目以降が本体。
//! 本体は `USER TTY FROM LOGIN@ IDLE WHAT...` で WHAT は空白を含むため余りを束ねる。

use serde::Serialize;

use crate::privileged;

/// 1行目のヘッダ。Frontend の `WSummary` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WSummary {
    pub headline: String,
    pub load_avg_1: Option<f64>,
    pub load_avg_5: Option<f64>,
    pub load_avg_15: Option<f64>,
}

/// ログイン中ユーザの1行。Frontend の `WEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WEntry {
    pub user: String,
    pub tty: String,
    pub from: String,
    pub login: String,
    pub idle: String,
    pub what: String,
}

/// `get_w` の返却値。Frontend の `WSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct WSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub summary: WSummary,
    pub users: Vec<WEntry>,
    pub count: usize,
    pub stderr: String,
}

/// `w` (引数なし)
pub fn current() -> Result<WSnapshot, String> {
    let preview = "w".to_string();
    let output = privileged::execute_plain("/usr/bin/w", &[], preview.clone())?;
    let (summary, users) = parse_output(&output.stdout);
    Ok(WSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        summary,
        count: users.len(),
        users,
        stderr: output.stderr.trim().to_string(),
    })
}

fn parse_output(stdout: &str) -> (WSummary, Vec<WEntry>) {
    let mut lines = stdout.lines();
    let headline = lines.next().unwrap_or("").trim().to_string();
    // 2行目は `USER TTY FROM ...` の列名なので読み捨てる
    lines.next();
    let summary = WSummary {
        load_avg_1: None,
        load_avg_5: None,
        load_avg_15: None,
        headline: headline.clone(),
    };
    let summary = parse_loads(&headline, summary);
    let users = lines.filter_map(parse_line).collect();
    (summary, users)
}

/// `... load averages: 9.48 11.49 10.52` を拾う。無ければ None のまま。
fn parse_loads(headline: &str, mut summary: WSummary) -> WSummary {
    let Some(pos) = headline.find("load averages:") else {
        return summary;
    };
    let mut loads = headline[pos + "load averages:".len()..]
        .split_whitespace()
        .filter_map(|token| token.trim_end_matches(',').parse::<f64>().ok());
    summary.load_avg_1 = loads.next();
    summary.load_avg_5 = loads.next();
    summary.load_avg_15 = loads.next();
    summary
}

/// `USER TTY FROM LOGIN@ IDLE WHAT...` をパースする。WHAT 無しも許す。
fn parse_line(line: &str) -> Option<WEntry> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    if tokens.len() < 5 {
        return None;
    }
    Some(WEntry {
        user: tokens[0].to_string(),
        tty: tokens[1].to_string(),
        from: tokens[2].to_string(),
        login: tokens[3].to_string(),
        idle: tokens[4].to_string(),
        what: tokens.get(5..).unwrap_or(&[]).join(" "),
    })
}

#[cfg(test)]
mod tests {
    use super::{current, parse_output};

    const SAMPLE: &str = "\
12:09  up 19 days, 19:06, 8 users, load averages: 9.48 11.49 10.52
USER       TTY      FROM    LOGIN@  IDLE WHAT
aoyamakoya console  -      Wed12   2days -
aoyamakoya s004     -      Thu09   26:50 /Users/aoyamakoya/.local/bin/agent --u
aoyamakoya s005     -      Fri15       1 -/bin/zsh -l
";

    #[test]
    fn parses_header_and_rows() {
        let (summary, users) = parse_output(SAMPLE);
        assert_eq!(summary.load_avg_1, Some(9.48));
        assert_eq!(summary.load_avg_5, Some(11.49));
        assert_eq!(summary.load_avg_15, Some(10.52));
        assert!(summary.headline.contains("load averages:"));
        assert_eq!(users.len(), 3);
        assert_eq!(users[0].user, "aoyamakoya");
        assert_eq!(users[0].tty, "console");
        assert_eq!(users[0].what, "-");
        assert_eq!(users[1].what, "/Users/aoyamakoya/.local/bin/agent --u");
        assert_eq!(users[2].idle, "1");
    }

    #[test]
    fn tolerates_missing_header() {
        let (summary, users) = parse_output("");
        assert_eq!(summary.load_avg_1, None);
        assert!(users.is_empty());
    }

    #[test]
    fn runs_w() {
        let result = current().expect("w");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.command, "w");
        assert_eq!(result.count, result.users.len());
        assert!(!result.summary.headline.is_empty());
    }
}
