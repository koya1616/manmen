//! `top -l 1` の定義・実行・出力パース。管理者権限は不要。
//! 対話モードの `top` はGUIから扱えないため、ロギングモード (`-l 1`) の
//! スナップショット取得に限定する (`manpage.rs` と同じ `execute_plain` 経路)。
//!
//! `top` の生テキストをそのまま返さず、サマリーとプロセス一覧にパースした
//! `TopSnapshot` を返す。Frontend はテーブルUIで表示する。

use serde::Serialize;

use crate::privileged;

/// 許可するソートキー (`top -o`) のサブセット。`man top` の `-o key` に準拠。
pub const ALLOWED_SORT_KEYS: &[&str] = &["cpu", "mem", "time", "pid", "command"];

pub const DEFAULT_SORT_KEY: &str = "cpu";
pub const DEFAULT_COUNT: u32 = 20;
pub const MAX_COUNT: u32 = 100;

/// サマリー部 (`top` 出力のプロセス表より上の行) の抜粋。
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
}

/// プロセス表の1行分 (表示に使う列だけ抜き出す)。
/// Frontend の `TopProcess` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct TopProcess {
    pub pid: u32,
    pub command: String,
    pub cpu: f32,
    pub time: String,
    pub threads: u32,
    pub ports: u32,
    pub mem: String,
    pub state: String,
    pub user: String,
}

/// `get_top` の返却値。Frontend の `TopSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct TopSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub summary: TopSummary,
    pub processes: Vec<TopProcess>,
    pub stderr: String,
}

/// `top -l 1 -o <sort> -n <count>`
pub struct Top {
    pub sort_key: String,
    pub count: u32,
}

impl Top {
    pub fn new(sort_key: &str, count: u32) -> Result<Self, String> {
        let sort_key = sort_key.trim().to_string();
        validate_sort_key(&sort_key)?;
        validate_count(count)?;
        Ok(Self { sort_key, count })
    }

    pub fn preview(&self) -> String {
        format!("top -l 1 -o {} -n {}", self.sort_key, self.count)
    }

    pub fn run(&self) -> Result<TopSnapshot, String> {
        let preview = self.preview();
        let count = self.count.to_string();
        let output = privileged::execute_plain(
            "/usr/bin/top",
            &["-l", "1", "-o", &self.sort_key, "-n", &count],
            preview.clone(),
        )?;
        let (summary, processes) = parse_snapshot(&output.stdout);
        Ok(TopSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            summary,
            processes,
            stderr: output.stderr,
        })
    }
}

fn validate_sort_key(sort_key: &str) -> Result<(), String> {
    if ALLOWED_SORT_KEYS.contains(&sort_key) {
        Ok(())
    } else {
        Err(format!("ソートキーが不正です: {sort_key}"))
    }
}

fn validate_count(count: u32) -> Result<(), String> {
    if (1..=MAX_COUNT).contains(&count) {
        Ok(())
    } else {
        Err(format!("表示件数は 1〜{MAX_COUNT} の範囲で指定してください"))
    }
}

pub fn get_snapshot(sort_key: String, count: u32) -> Result<TopSnapshot, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (sort_key, count);
        return Err("top is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return Top::new(&sort_key, count).and_then(|cmd| cmd.run());
}

/// `top -l 1` の stdout 全体をサマリーとプロセス一覧に分けてパースする。
/// 見出し行 (`PID ...` で始まる行) より上がサマリー、以降の行がプロセス表。
fn parse_snapshot(stdout: &str) -> (TopSummary, Vec<TopProcess>) {
    let lines: Vec<&str> = stdout.lines().collect();
    let header_idx = lines
        .iter()
        .position(|l| l.trim_start_matches(' ').starts_with("PID"));
    let (summary_lines, table_lines) = match header_idx {
        Some(i) => (&lines[..i], &lines[i..]),
        None => (&lines[..], &[][..]),
    };
    let summary = parse_summary(summary_lines);
    let processes = parse_processes(table_lines);
    (summary, processes)
}

fn parse_summary(lines: &[&str]) -> TopSummary {
    let mut summary = TopSummary::default();
    for line in lines {
        let line = line.trim();
        if line.starts_with("Processes:") {
            // 例: "Processes: 552 total, 5 running, 547 sleeping, 4129 threads"
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
            // 例: "Load Avg: 5.23, 8.13, 10.70"
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
            // 例: "CPU usage: 32.29% user, 26.28% sys, 41.41% idle"
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
            // 例: "2026/10/01 10:04:05" (ロギングモードの時刻行)
            summary.timestamp = line.to_string();
        }
    }
    summary
}

/// プロセス表 (見出し行 + データ行) をパースする。
/// COMMAND 名に空白を含まない前提で、見出しの列位置を基準に抜き出す。
fn parse_processes(lines: &[&str]) -> Vec<TopProcess> {
    if lines.is_empty() {
        return Vec::new();
    }
    let header: Vec<&str> = lines[0].split_whitespace().collect();
    let col = |name: &str| header.iter().position(|h| *h == name);
    let (i_pid, i_cmd, i_cpu, i_time, i_th, i_ports, i_mem, i_state, i_user) = match (
        col("PID"),
        col("COMMAND"),
        col("%CPU"),
        col("TIME"),
        col("#TH"),
        col("#PORTS"),
        col("MEM"),
        col("STATE"),
        col("USER"),
    ) {
        (Some(a), Some(b), Some(c), Some(d), Some(e), Some(f), Some(g), Some(h), Some(i)) => {
            (a, b, c, d, e, f, g, h, i)
        }
        _ => return Vec::new(),
    };

    lines[1..]
        .iter()
        .filter_map(|line| {
            let t: Vec<&str> = line.split_whitespace().collect();
            let get = |i: usize| t.get(i).map(|s| s.to_string());
            Some(TopProcess {
                pid: t.get(i_pid)?.parse().ok()?,
                command: get(i_cmd)?,
                cpu: t.get(i_cpu)?.parse().ok()?,
                time: get(i_time)?,
                threads: t.get(i_th)?.parse().ok()?,
                ports: t.get(i_ports)?.parse().ok()?,
                mem: get(i_mem)?,
                state: get(i_state)?,
                user: get(i_user)?,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Top, MAX_COUNT, parse_processes, parse_snapshot, parse_summary};

    const SAMPLE: &str = "Processes: 552 total, 5 running, 547 sleeping, 4129 threads \n\
         2026/10/01 10:04:05\n\
         Load Avg: 5.23, 8.13, 10.70 \n\
         CPU usage: 32.29% user, 26.28% sys, 41.41% idle \n\
         PhysMem: 15G used (2991M wired, 6774M compressor), 77M unused.\n\
         \n\
         PID    COMMAND          %CPU TIME     #TH #WQ #PORTS MEM   PURG CMPRS STATE    USER\n\
         97040  circleci-yaml-la 12.5  00:07.06 13  0   34     11M   0B   9536K sleeping aoyamakoya\n\
         96743  coreauthd        0.0  00:00.70 2   1   75     4960K 0B   4592K sleeping root\n";

    #[test]
    fn previews_top_snapshot_command() {
        let top = Top::new("cpu", 20).unwrap();
        assert_eq!(top.preview(), "top -l 1 -o cpu -n 20");
        let top = Top::new("mem", 10).unwrap();
        assert_eq!(top.preview(), "top -l 1 -o mem -n 10");
    }

    #[test]
    fn rejects_invalid_sort_key() {
        assert!(Top::new("cpu", 20).is_ok());
        assert!(Top::new("mem", 20).is_ok());
        assert!(Top::new("time", 20).is_ok());
        assert!(Top::new("pid", 20).is_ok());
        assert!(Top::new("command", 20).is_ok());
        assert!(Top::new("-o", 20).is_err());
        assert!(Top::new("", 20).is_err());
        assert!(Top::new("cpu;rm -rf /", 20).is_err());
    }

    #[test]
    fn rejects_out_of_range_count() {
        assert!(Top::new("cpu", 1).is_ok());
        assert!(Top::new("cpu", MAX_COUNT).is_ok());
        assert!(Top::new("cpu", 0).is_err());
        assert!(Top::new("cpu", MAX_COUNT + 1).is_err());
    }

    #[test]
    fn parses_summary_values() {
        let lines: Vec<&str> = SAMPLE.lines().collect();
        let summary = parse_summary(&lines[..6]);
        assert_eq!(summary.timestamp, "2026/10/01 10:04:05");
        assert_eq!(summary.processes_total, Some(552));
        assert_eq!(summary.processes_running, Some(5));
        assert_eq!(summary.processes_sleeping, Some(547));
        assert_eq!(summary.threads, Some(4129));
        assert_eq!(summary.load_avg_1, Some(5.23));
        assert_eq!(summary.load_avg_5, Some(8.13));
        assert_eq!(summary.load_avg_15, Some(10.70));
        assert_eq!(summary.cpu_user, Some(32.29));
        assert_eq!(summary.cpu_sys, Some(26.28));
        assert_eq!(summary.cpu_idle, Some(41.41));
        assert!(summary.physmem.starts_with("PhysMem: 15G used"));
    }

    #[test]
    fn parses_process_rows() {
        let lines: Vec<&str> = SAMPLE.lines().collect();
        let procs = parse_processes(&lines[6..]);
        assert_eq!(procs.len(), 2);
        assert_eq!(procs[0].pid, 97040);
        assert_eq!(procs[0].command, "circleci-yaml-la");
        assert_eq!(procs[0].cpu, 12.5);
        assert_eq!(procs[0].threads, 13);
        assert_eq!(procs[0].ports, 34);
        assert_eq!(procs[0].mem, "11M");
        assert_eq!(procs[0].state, "sleeping");
        assert_eq!(procs[0].user, "aoyamakoya");
        assert_eq!(procs[1].pid, 96743);
        assert_eq!(procs[1].user, "root");
    }

    #[test]
    fn parses_full_snapshot() {
        let (summary, procs) = parse_snapshot(SAMPLE);
        assert_eq!(summary.processes_total, Some(552));
        assert_eq!(procs.len(), 2);
    }

    #[test]
    fn tolerates_missing_header() {
        let (summary, procs) = parse_snapshot("something unexpected\n");
        assert_eq!(summary.processes_total, None);
        assert!(procs.is_empty());
    }
}
