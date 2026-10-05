//! `ps` の定義・実行・出力パース。管理者権限は不要。
//! 対話操作はなく、許可した表示列・並び替え・絞り込みだけのスナップショットに限る。
//! 常に `-A` (全プロセス) を付け、`-o` の列だけを許可する。
//!
//! 空白を含む `command` 列は必ず末尾に回す (行パースが崩れないため)。

use serde::{Deserialize, Serialize};

use crate::common::parse::parse_table;
use crate::common::validate::{parse_pids as parse_pids_common, validate_user};
use crate::privileged;

/// `man ps` の `-o` キーワードのうち日常的なものに限る。
/// エイリアスはコマンド注入を避けるため受け付けない。
pub const ALLOWED_COLUMNS: &[&str] = &[
    "pid", "ppid", "pgid", "user", "%cpu", "%mem", "rss", "vsz", "time", "etime", "state",
    "tty", "nice", "comm", "command", "start",
];

/// 空白を含む可能性のある列。末尾固定にする。
const TRAILING_COLUMNS: &[&str] = &["command"];

pub const DEFAULT_COLUMNS: &[&str] =
    &["pid", "user", "%cpu", "%mem", "rss", "time", "state", "command"];

pub const ALLOWED_SORTS: &[&str] = &["none", "cpu", "mem"];
const MAX_PIDS: usize = 16;
const MAX_COLUMNS: usize = 16;

/// Frontend の `getPs` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PsQuery {
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default)]
    pub columns: Vec<String>,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub pids: String,
}

fn default_sort() -> String {
    "none".to_string()
}

/// `get_ps` の返却値。Frontend の `PsSnapshot` とフィールドを一致させること。
/// `columns` は要求したキーワード順、その通りに `rows` を切って返す。
#[derive(Debug, Clone, Serialize)]
pub struct PsSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub count: usize,
    pub stderr: String,
}

/// `ps -A` に、検証済みのオプションだけを足したもの。
pub struct Ps {
    args: Vec<String>,
    columns: Vec<String>,
}

impl Ps {
    pub fn new(query: PsQuery) -> Result<Self, String> {
        let (args, columns) = build_args(query)?;
        Ok(Self { args, columns })
    }

    pub fn preview(&self) -> String {
        format!("ps {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<PsSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("ps", &refs, preview.clone())?;
        let rows = parse_rows(&output.stdout, self.columns.len());
        Ok(PsSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            columns: self.columns.clone(),
            rows: rows.clone(),
            count: rows.len(),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: PsQuery) -> Result<PsSnapshot, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("ps is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return Ps::new(query).and_then(|cmd| cmd.run());
}

fn build_args(query: PsQuery) -> Result<(Vec<String>, Vec<String>), String> {
    let sort = query.sort.trim();
    validate_sort(sort)?;
    let user = query.user.trim();
    validate_user(user)?;
    let pids = parse_pids(&query.pids)?;
    let columns = normalize_columns(&query.columns)?;

    let mut args = vec!["-A".to_string()];
    match sort {
        "cpu" => args.push("-r".to_string()),
        "mem" => args.push("-m".to_string()),
        _ => {}
    }
    if !user.is_empty() {
        args.push("-U".to_string());
        args.push(user.to_string());
    }
    if !pids.is_empty() {
        args.push("-p".to_string());
        args.push(pids.join(","));
    }
    args.push("-o".to_string());
    args.push(columns.join(","));
    Ok((args, columns))
}

fn validate_sort(sort: &str) -> Result<(), String> {
    if ALLOWED_SORTS.contains(&sort) {
        Ok(())
    } else {
        Err(format!("並び替えが不正です: {sort}"))
    }
}

fn parse_pids(raw: &str) -> Result<Vec<String>, String> {
    parse_pids_common(raw, MAX_PIDS)
}

fn normalize_columns(columns: &[String]) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for column in columns {
        let column = column.trim().to_lowercase();
        if column.is_empty() {
            continue;
        }
        if !ALLOWED_COLUMNS.contains(&column.as_str()) {
            return Err(format!("表示列が不正です: {column}"));
        }
        if !out.iter().any(|item: &String| item == &column) {
            out.push(column);
        }
    }
    if out.is_empty() {
        return Err("表示列を1つ以上選んでください".to_string());
    }
    if out.len() > MAX_COLUMNS {
        return Err(format!("表示列は {MAX_COLUMNS} 個までです"));
    }
    // 空白を含む列は末尾に回す
    let (trailing, head): (Vec<String>, Vec<String>) =
        out.into_iter().partition(|c| TRAILING_COLUMNS.contains(&c.as_str()));
    Ok(head.into_iter().chain(trailing).collect())
}

/// 先頭の見出し行を除き、列数に合わせて行を切る。
/// 末尾列 (`command`) は空白を含むため残り全部を1セルにする。
fn parse_rows(stdout: &str, width: usize) -> Vec<Vec<String>> {
    parse_table(stdout, width)
}

#[cfg(test)]
mod tests {
    use super::{parse_rows, Ps, PsQuery, MAX_COLUMNS, MAX_PIDS};

    const SAMPLE: &str = "  PID USER     %CPU %MEM COMMAND\n\
        1 root       0.0  0.0 /sbin/launchd\n\
        42 alice     1.2  0.3 /usr/bin/python3 run server --port 8080\n";

    fn base_query() -> PsQuery {
        PsQuery {
            sort: "none".to_string(),
            columns: vec![
                "pid".to_string(),
                "user".to_string(),
                "%cpu".to_string(),
                "%mem".to_string(),
                "command".to_string(),
            ],
            user: String::new(),
            pids: String::new(),
        }
    }

    #[test]
    fn previews_default_snapshot() {
        let ps = Ps::new(base_query()).unwrap();
        assert_eq!(ps.preview(), "ps -A -o pid,user,%cpu,%mem,command");
    }

    #[test]
    fn previews_sort_and_filters() {
        let mut query = base_query();
        query.sort = "cpu".to_string();
        query.user = "root".to_string();
        query.pids = "1, 2,1".to_string();
        let ps = Ps::new(query).unwrap();
        assert_eq!(
            ps.preview(),
            "ps -A -r -U root -p 1,2 -o pid,user,%cpu,%mem,command"
        );

        let mut query = base_query();
        query.sort = "mem".to_string();
        let ps = Ps::new(query).unwrap();
        assert!(ps.preview().starts_with("ps -A -m -o "));
    }

    #[test]
    fn moves_command_last() {
        let mut query = base_query();
        query.columns = vec!["command".to_string(), "pid".to_string()];
        let ps = Ps::new(query).unwrap();
        assert_eq!(ps.preview(), "ps -A -o pid,command");
    }

    #[test]
    fn rejects_unknown_options() {
        let mut query = base_query();
        query.sort = "uptime".to_string();
        assert!(Ps::new(query).is_err());

        let mut query = base_query();
        query.columns = vec!["pid".to_string(), "args;rm".to_string()];
        assert!(Ps::new(query).is_err());

        let mut query = base_query();
        query.columns = vec![];
        assert!(Ps::new(query).is_err());

        let mut query = base_query();
        query.columns = vec!["pid".to_string(); MAX_COLUMNS + 1];
        let ps = Ps::new(query).unwrap();
        assert_eq!(ps.preview(), "ps -A -o pid");

        let mut query = base_query();
        query.user = "root;id".to_string();
        assert!(Ps::new(query).is_err());

        let mut query = base_query();
        query.pids = "1,-1".to_string();
        assert!(Ps::new(query).is_err());

        let mut query = base_query();
        query.pids = (0..=MAX_PIDS).map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        assert!(Ps::new(query).is_err());
    }

    #[test]
    fn parses_rows_keeping_command_intact() {
        let rows = parse_rows(SAMPLE, 5);
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec!["1", "root", "0.0", "0.0", "/sbin/launchd"]);
        assert_eq!(
            rows[1],
            vec!["42", "alice", "1.2", "0.3", "/usr/bin/python3 run server --port 8080"]
        );
    }

    #[test]
    fn tolerates_empty_output() {
        let rows = parse_rows("  PID USER COMMAND\n", 3);
        assert!(rows.is_empty());
    }
}
