//! `top -l 1` の定義・実行・出力パース。管理者権限は不要。
//! 対話モードは扱えないので、ロギングモード (`-l 1`) の1回分に限る。
//! 並び替え・絞り込み・集計・表示列は `man top` のスナップショット向けオプションだけを許可する。
//!
//! プロセス表は見出しに合わせた行として返す。`-stats` で列が変わっても表示できる。

use serde::{Deserialize, Serialize};

use crate::privileged;

/// `man top` の `-o` / `-stats` で使えるキー。エイリアスはコマンド注入を避けるため受け付けない。
pub const ALLOWED_KEYS: &[&str] = &[
    "pid",
    "command",
    "cpu",
    "cpu_me",
    "cpu_others",
    "csw",
    "time",
    "threads",
    "ports",
    "mregion",
    "mem",
    "rprvt",
    "purg",
    "vsize",
    "vprvt",
    "kprvt",
    "kshrd",
    "pgrp",
    "ppid",
    "state",
    "uid",
    "wq",
    "faults",
    "cow",
    "user",
    "msgsent",
    "msgrecv",
    "sysbsd",
    "sysmach",
    "pageins",
    "boosts",
    "instrs",
    "cycles",
    "jetpri",
];

pub const DEFAULT_SORT_KEY: &str = "cpu";
pub const DEFAULT_COUNT: u32 = 20;
pub const MAX_COUNT: u32 = 100;
const MAX_PIDS: usize = 16;
const MAX_STATS: usize = 16;
const MIN_NCOLS: u32 = 40;
const MAX_NCOLS: u32 = 400;

/// Frontend の `getTop` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TopQuery {
    pub sort_key: String,
    #[serde(default)]
    pub sort_order: String,
    #[serde(default)]
    pub secondary_key: String,
    pub count: u32,
    #[serde(default = "default_count_mode")]
    pub count_mode: String,
    #[serde(default)]
    pub no_frameworks: bool,
    #[serde(default)]
    pub memory_map: bool,
    #[serde(default)]
    pub swap: bool,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub pids: String,
    #[serde(default)]
    pub stats: Vec<String>,
    #[serde(default)]
    pub ncols: Option<u32>,
}

fn default_count_mode() -> String {
    "n".to_string()
}

/// サマリー部の抜粋。知っている行以外は `extra` に残す (`-S` の Swap など)。
/// Frontend の `TopSummary` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct TopSummary {
    pub timestamp: String,
    pub processes_total: Option<u32>,
    pub processes_running: Option<u32>,
    pub processes_sleeping: Option<u32>,
    pub threads: Option<u32>,
    pub load_avg_1: Option<f32>,
    pub load_avg_5: Option<f32>,
    pub load_avg_15: Option<f32>,
    pub cpu_user: Option<f32>,
    pub cpu_sys: Option<f32>,
    pub cpu_idle: Option<f32>,
    pub physmem: String,
    pub extra: Vec<String>,
}

/// `get_top` の返却値。Frontend の `TopSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct TopSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub summary: TopSummary,
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub stderr: String,
}

/// `top -l 1` に、検証済みのオプションだけを足したもの。
pub struct Top {
    args: Vec<String>,
}

impl Top {
    pub fn new(query: TopQuery) -> Result<Self, String> {
        Ok(Self {
            args: build_args(query)?,
        })
    }

    pub fn preview(&self) -> String {
        format!("top {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<TopSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/usr/bin/top", &refs, preview.clone())?;
        let (summary, columns, rows) = parse_snapshot(&output.stdout);
        Ok(TopSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            summary,
            columns,
            rows,
            stderr: output.stderr,
        })
    }
}

pub fn get_snapshot(query: TopQuery) -> Result<TopSnapshot, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("top is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return Top::new(query).and_then(|cmd| cmd.run());
}

fn build_args(query: TopQuery) -> Result<Vec<String>, String> {
    let sort_key = query.sort_key.trim();
    validate_key(sort_key)?;
    validate_order(&query.sort_order)?;
    let secondary = query.secondary_key.trim();
    if !secondary.is_empty() {
        validate_key(secondary)?;
    }
    validate_count(query.count)?;
    validate_count_mode(&query.count_mode)?;
    let user = query.user.trim();
    validate_user(user)?;
    let pids = parse_pids(&query.pids)?;
    let stats = normalize_stats(&query.stats)?;
    validate_ncols(query.ncols)?;

    let mut args = vec!["-l".to_string(), "1".to_string()];
    args.push("-o".to_string());
    args.push(format!("{}{sort_key}", query.sort_order));
    if !secondary.is_empty() {
        args.push("-O".to_string());
        args.push(secondary.to_string());
    }
    args.push("-n".to_string());
    args.push(query.count.to_string());
    if query.count_mode != "n" {
        args.push("-c".to_string());
        args.push(query.count_mode);
    }
    if query.no_frameworks {
        args.push("-F".to_string());
    }
    if query.memory_map {
        args.push("-r".to_string());
    }
    if query.swap {
        args.push("-S".to_string());
    }
    if !user.is_empty() {
        args.push("-user".to_string());
        args.push(user.to_string());
    }
    for pid in pids {
        args.push("-pid".to_string());
        args.push(pid);
    }
    if !stats.is_empty() {
        args.push("-stats".to_string());
        args.push(stats.join(","));
    }
    if let Some(ncols) = query.ncols {
        args.push("-ncols".to_string());
        args.push(ncols.to_string());
    }
    Ok(args)
}

fn validate_key(key: &str) -> Result<(), String> {
    if ALLOWED_KEYS.contains(&key) {
        Ok(())
    } else {
        Err(format!("ソートキーが不正です: {key}"))
    }
}

fn validate_order(order: &str) -> Result<(), String> {
    if matches!(order, "" | "+" | "-") {
        Ok(())
    } else {
        Err("並び順が不正です".to_string())
    }
}

fn validate_count(count: u32) -> Result<(), String> {
    if (1..=MAX_COUNT).contains(&count) {
        Ok(())
    } else {
        Err(format!("表示件数は 1〜{MAX_COUNT} の範囲で指定してください"))
    }
}

fn validate_count_mode(mode: &str) -> Result<(), String> {
    if matches!(mode, "n" | "a" | "d" | "e") {
        Ok(())
    } else {
        Err(format!("集計モードが不正です: {mode}"))
    }
}

fn validate_user(user: &str) -> Result<(), String> {
    if user.is_empty() {
        return Ok(());
    }
    let ok = user.len() <= 32
        && user
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'))
        && !user.starts_with('-');
    if ok {
        Ok(())
    } else {
        Err(format!("ユーザ名が不正です: {user}"))
    }
}

fn parse_pids(raw: &str) -> Result<Vec<String>, String> {
    let mut pids = Vec::new();
    for token in raw.split(|c: char| c == ',' || c.is_whitespace()) {
        if token.is_empty() {
            continue;
        }
        if token.parse::<u32>().is_err() || token.starts_with('+') || token.starts_with('-') {
            return Err(format!("プロセスIDが不正です: {token}"));
        }
        if !pids.contains(&token.to_string()) {
            pids.push(token.to_string());
        }
    }
    if pids.len() > MAX_PIDS {
        return Err(format!("プロセスIDは {MAX_PIDS} 件までです"));
    }
    Ok(pids)
}

fn normalize_stats(stats: &[String]) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for stat in stats {
        let stat = stat.trim();
        if stat.is_empty() {
            continue;
        }
        validate_key(stat)?;
        if !out.iter().any(|item: &String| item == stat) {
            out.push(stat.to_string());
        }
    }
    if out.len() > MAX_STATS {
        return Err(format!("表示列は {MAX_STATS} 個までです"));
    }
    Ok(out)
}

fn validate_ncols(ncols: Option<u32>) -> Result<(), String> {
    match ncols {
        None => Ok(()),
        Some(n) if (MIN_NCOLS..=MAX_NCOLS).contains(&n) => Ok(()),
        Some(_) => Err(format!("表示幅は {MIN_NCOLS}〜{MAX_NCOLS} の範囲で指定してください")),
    }
}

/// `top -l 1` の stdout 全体をサマリーとプロセス表に分けてパースする。
/// 見出し行 (`PID` で始まる行) より上がサマリー、以降が表。
fn parse_snapshot(stdout: &str) -> (TopSummary, Vec<String>, Vec<Vec<String>>) {
    let lines: Vec<&str> = stdout.lines().collect();
    let header_idx = lines
        .iter()
        .position(|line| line.trim_start_matches(' ').starts_with("PID"));
    let (summary_lines, table_lines) = match header_idx {
        Some(i) => (&lines[..i], &lines[i..]),
        None => (&lines[..], &[][..]),
    };
    let summary = parse_summary(summary_lines);
    let (columns, rows) = parse_table(table_lines);
    (summary, columns, rows)
}

fn parse_summary(lines: &[&str]) -> TopSummary {
    let mut summary = TopSummary::default();
    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            continue;
        } else if line.starts_with("Processes:") {
            for part in line.trim_start_matches("Processes:").split(',') {
                let tokens: Vec<&str> = part.split_whitespace().collect();
                if tokens.len() != 2 {
                    continue;
                }
                let n: Option<u32> = tokens[0].parse().ok();
                match tokens[1] {
                    "total" => summary.processes_total = n,
                    "running" => summary.processes_running = n,
                    "sleeping" => summary.processes_sleeping = n,
                    "threads" => summary.threads = n,
                    _ => {}
                }
            }
        } else if line.starts_with("Load Avg:") {
            let values: Vec<f32> = line
                .trim_start_matches("Load Avg:")
                .split(',')
                .filter_map(|v| v.trim().parse().ok())
                .collect();
            if values.len() == 3 {
                summary.load_avg_1 = Some(values[0]);
                summary.load_avg_5 = Some(values[1]);
                summary.load_avg_15 = Some(values[2]);
            }
        } else if line.starts_with("CPU usage:") {
            for part in line.trim_start_matches("CPU usage:").split(',') {
                let tokens: Vec<&str> = part.split_whitespace().collect();
                if tokens.len() != 2 {
                    continue;
                }
                let v: Option<f32> = tokens[0].trim_end_matches('%').parse().ok();
                match tokens[1] {
                    "user" => summary.cpu_user = v,
                    "sys" => summary.cpu_sys = v,
                    "idle" => summary.cpu_idle = v,
                    _ => {}
                }
            }
        } else if line.starts_with("PhysMem:") {
            summary.physmem = line.to_string();
        } else if summary.timestamp.is_empty()
            && line.contains('/')
            && line.contains(':')
            && !line.contains("Avg")
            && !line.contains("usage")
        {
            summary.timestamp = line.to_string();
        } else if line.contains(':') {
            summary.extra.push(line.to_string());
        }
    }
    summary
}

/// 見出しトークン数に揃えてデータ行を切る。COMMAND は空白を含まない。
fn parse_table(lines: &[&str]) -> (Vec<String>, Vec<Vec<String>>) {
    if lines.is_empty() {
        return (Vec::new(), Vec::new());
    }
    let columns: Vec<String> = lines[0].split_whitespace().map(str::to_string).collect();
    let width = columns.len();
    let rows = lines[1..]
        .iter()
        .filter_map(|line| {
            let cells: Vec<String> = line.split_whitespace().map(str::to_string).collect();
            if cells.len() < width {
                return None;
            }
            Some(cells.into_iter().take(width).collect())
        })
        .collect();
    (columns, rows)
}

#[cfg(test)]
mod tests {
    use super::{parse_snapshot, parse_summary, Top, TopQuery, MAX_COUNT};

    const SAMPLE: &str = "Processes: 552 total, 5 running, 547 sleeping, 4129 threads \n\
         2026/10/01 10:04:05\n\
         Load Avg: 5.23, 8.13, 10.70 \n\
         CPU usage: 32.29% user, 26.28% sys, 41.41% idle \n\
         SharedLibs: 447M resident, 101M data, 79M linkedit.\n\
         PhysMem: 15G used (2991M wired, 6774M compressor), 77M unused.\n\
         \n\
         PID    COMMAND          %CPU TIME     #TH #WQ #PORTS MEM   PURG CMPRS STATE    USER\n\
         97040  circleci-yaml-la 12.5  00:07.06 13  0   34     11M   0B   9536K sleeping aoyamakoya\n\
         96743  coreauthd        0.0  00:00.70 2   1   75     4960K 0B   4592K sleeping root\n";

    fn base_query() -> TopQuery {
        TopQuery {
            sort_key: "cpu".to_string(),
            sort_order: String::new(),
            secondary_key: String::new(),
            count: 20,
            count_mode: "n".to_string(),
            no_frameworks: false,
            memory_map: false,
            swap: false,
            user: String::new(),
            pids: String::new(),
            stats: Vec::new(),
            ncols: None,
        }
    }

    #[test]
    fn previews_selected_options() {
        let top = Top::new(base_query()).unwrap();
        assert_eq!(top.preview(), "top -l 1 -o cpu -n 20");

        let mut query = base_query();
        query.sort_key = "mem".to_string();
        query.sort_order = "+".to_string();
        query.secondary_key = "time".to_string();
        query.count = 5;
        query.count_mode = "a".to_string();
        query.no_frameworks = true;
        query.memory_map = true;
        query.swap = true;
        query.user = "root".to_string();
        query.pids = "1, 2,1".to_string();
        query.stats = vec!["pid".to_string(), "cpu".to_string()];
        query.ncols = Some(80);
        let top = Top::new(query).unwrap();
        assert_eq!(
            top.preview(),
            "top -l 1 -o +mem -O time -n 5 -c a -F -r -S -user root -pid 1 -pid 2 -stats pid,cpu -ncols 80"
        );
    }

    #[test]
    fn rejects_unknown_options() {
        let mut query = base_query();
        query.sort_key = "-o".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.sort_order = "up".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.secondary_key = "sleepnow".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.count_mode = "x".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.user = "root;id".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.pids = "1,-1".to_string();
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.stats = vec!["cpu".to_string(), "not-a-key".to_string()];
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.ncols = Some(10);
        assert!(Top::new(query).is_err());

        let mut query = base_query();
        query.count = MAX_COUNT + 1;
        assert!(Top::new(query).is_err());
    }

    #[test]
    fn parses_summary_and_flexible_rows() {
        let lines: Vec<&str> = SAMPLE.lines().collect();
        let summary = parse_summary(&lines[..7]);
        assert_eq!(summary.timestamp, "2026/10/01 10:04:05");
        assert_eq!(summary.processes_total, Some(552));
        assert_eq!(summary.load_avg_1, Some(5.23));
        assert_eq!(summary.cpu_user, Some(32.29));
        assert!(summary.physmem.starts_with("PhysMem: 15G used"));
        assert_eq!(
            summary.extra,
            vec!["SharedLibs: 447M resident, 101M data, 79M linkedit.".to_string()]
        );

        let (summary, columns, rows) = parse_snapshot(SAMPLE);
        assert_eq!(summary.processes_total, Some(552));
        assert_eq!(columns[0], "PID");
        assert_eq!(columns[2], "%CPU");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0][0], "97040");
        assert_eq!(rows[0][1], "circleci-yaml-la");
        assert_eq!(rows[1][columns.len() - 1], "root");
    }

    #[test]
    fn tolerates_missing_header() {
        let (summary, columns, rows) = parse_snapshot("something unexpected\n");
        assert_eq!(summary.processes_total, None);
        assert!(columns.is_empty());
        assert!(rows.is_empty());
    }
}
