//! `du` の定義・実行・出力パース。管理者権限は不要。
//! 大きなフォルダでは数分かかるため、打ち切り時間 (5〜120 秒) を必須にし、超えたら途中結果を返す。
//! 返す件数が膨らまないよう、深さは 0〜3 に制限し、大きい順に MAX_ENTRIES 件までに絞る。
//! 単位は Frontend で変換するため常に `-k` で取得する。シンボリックリンク追従 (`-H`/`-L`)・
//! 除外パターン (`-I`)・ハードリンクの重複計上 (`-l`) は扱わない。

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::common::validate::resolve_existing_path;
use crate::privileged;

pub const DEFAULT_DEPTH: u32 = 1;
pub const MAX_DEPTH: u32 = 3;
pub const DEFAULT_TIMEOUT_SECS: u32 = 30;
pub const MIN_TIMEOUT_SECS: u32 = 5;
pub const MAX_TIMEOUT_SECS: u32 = 120;
pub const MAX_ENTRIES: usize = 1000;
pub const MAX_ERROR_LINES: usize = 20;

/// Frontend の `getDu` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuQuery {
    pub path: String,
    #[serde(default = "default_depth")]
    pub depth: u32,
    #[serde(default)]
    pub all_files: bool,
    #[serde(default)]
    pub one_fs: bool,
    #[serde(default)]
    pub apparent: bool,
    #[serde(default = "default_timeout")]
    pub timeout_secs: u32,
}

fn default_depth() -> u32 {
    DEFAULT_DEPTH
}

fn default_timeout() -> u32 {
    DEFAULT_TIMEOUT_SECS
}

/// 1件。Frontend の `DiskUsageItem` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DuItem {
    pub size_kb: u64,
    pub path: String,
    /// 指定パスからの深さ (指定パス自身は 0)
    pub depth: u32,
}

/// `get_du` の返却値。Frontend の `DiskUsageSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct DuSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub root: String,
    /// 指定パス全体のサイズ。打ち切りで最後まで数えられなかった場合は None
    pub total_kb: Option<u64>,
    pub items: Vec<DuItem>,
    pub total_items: usize,
    pub timed_out: bool,
    pub error_count: usize,
    pub stderr: String,
}

pub struct Du {
    root: String,
    args: Vec<String>,
    timeout: Duration,
}

impl Du {
    pub fn new(query: DuQuery) -> Result<Self, String> {
        if query.depth > MAX_DEPTH {
            return Err(format!("深さは 0〜{MAX_DEPTH} です"));
        }
        if !(MIN_TIMEOUT_SECS..=MAX_TIMEOUT_SECS).contains(&query.timeout_secs) {
            return Err(format!(
                "打ち切り時間は {MIN_TIMEOUT_SECS}〜{MAX_TIMEOUT_SECS} 秒です"
            ));
        }
        let root = resolve_existing_path(&query.path)?;
        let mut args: Vec<String> = vec!["-k".to_string()];
        if query.apparent {
            args.push("-A".to_string());
        }
        if query.one_fs {
            args.push("-x".to_string());
        }
        if query.all_files {
            args.push("-a".to_string());
        }
        args.push("-d".to_string());
        args.push(query.depth.to_string());
        args.push(root.clone());
        Ok(Self {
            root,
            args,
            timeout: Duration::from_secs(u64::from(query.timeout_secs)),
        })
    }

    pub fn preview(&self) -> String {
        format!("du {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<DuSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let (output, timed_out) =
            privileged::execute_plain_with_timeout("/usr/bin/du", &refs, preview.clone(), self.timeout)?;
        let mut items = parse_items(&output.stdout, &self.root);
        let total_kb = items.iter().find(|item| item.depth == 0).map(|item| item.size_kb);
        items.sort_by(|a, b| b.size_kb.cmp(&a.size_kb).then_with(|| a.path.cmp(&b.path)));
        let total_items = items.len();
        items.truncate(MAX_ENTRIES);
        let errors: Vec<&str> = output
            .stderr
            .lines()
            .filter(|line| line.starts_with("du: "))
            .collect();
        Ok(DuSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            root: self.root.clone(),
            total_kb,
            items,
            total_items,
            timed_out,
            error_count: errors.len(),
            // 読めないフォルダが大量にあると stderr が膨らむため先頭だけ返す
            stderr: errors
                .iter()
                .take(MAX_ERROR_LINES)
                .copied()
                .collect::<Vec<_>>()
                .join("\n"),
        })
    }
}

pub fn get_snapshot(query: DuQuery) -> Result<DuSnapshot, String> {
    Du::new(query).and_then(|cmd| cmd.run())
}

/// `12345\t/path/to/dir` をパースする。パスは空白を含みうるので最初のタブで切る。
fn parse_items(stdout: &str, root: &str) -> Vec<DuItem> {
    let root_trimmed = root.trim_end_matches('/');
    stdout
        .lines()
        .filter_map(|line| {
            let (size, path) = line.split_once('\t')?;
            let size_kb = size.trim().parse::<u64>().ok()?;
            let relative = path
                .strip_prefix(root_trimmed)
                .unwrap_or(path)
                .trim_start_matches('/');
            let depth = if relative.is_empty() {
                0
            } else {
                relative.split('/').count() as u32
            };
            Some(DuItem {
                size_kb,
                path: path.to_string(),
                depth,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{Du, DuQuery};

    fn base_query(path: &str) -> DuQuery {
        DuQuery {
            path: path.to_string(),
            depth: 1,
            all_files: false,
            one_fs: false,
            apparent: false,
            timeout_secs: 30,
        }
    }

    #[test]
    fn previews_queries() {
        let du = Du::new(base_query("/tmp")).unwrap();
        assert_eq!(du.preview(), "du -k -d 1 /tmp");

        let mut query = base_query("/tmp");
        query.depth = 2;
        query.all_files = true;
        query.one_fs = true;
        query.apparent = true;
        assert_eq!(Du::new(query).unwrap().preview(), "du -k -A -x -a -d 2 /tmp");
    }

    #[test]
    fn expands_home() {
        let home = std::env::var("HOME").unwrap();
        let du = Du::new(base_query("~")).unwrap();
        assert_eq!(du.preview(), format!("du -k -d 1 {home}"));
    }

    #[test]
    fn rejects_bad_input() {
        for path in ["", "-a", "tmp", "/no/such/path/manmen"] {
            assert!(Du::new(base_query(path)).is_err(), "{path:?}");
        }
        let mut query = base_query("/tmp");
        query.depth = 4;
        assert!(Du::new(query).is_err());
        let mut query = base_query("/tmp");
        query.timeout_secs = 1;
        assert!(Du::new(query).is_err());
    }

    #[test]
    fn parses_items_with_depth_and_spaces() {
        let items = super::parse_items(
            "8\t/data/My Docs/a b\n20\t/data/My Docs\n4\t/data/x\n40\t/data\n",
            "/data/",
        );
        assert_eq!(items.len(), 4);
        assert_eq!(items[0].path, "/data/My Docs/a b");
        assert_eq!(items[0].depth, 2);
        assert_eq!(items[1].depth, 1);
        assert_eq!(items[3].depth, 0);
        assert_eq!(items[3].size_kb, 40);
    }

    #[test]
    fn runs_du_on_temp_dir() {
        let dir = std::env::temp_dir().join(format!("manmen-du-test-{}", std::process::id()));
        std::fs::create_dir_all(dir.join("big")).unwrap();
        std::fs::write(dir.join("big/file"), vec![0u8; 64 * 1024]).unwrap();
        std::fs::create_dir_all(dir.join("small")).unwrap();

        let result = super::get_snapshot(base_query(dir.to_str().unwrap())).expect("du");
        let _ = std::fs::remove_dir_all(&dir);
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(!result.timed_out);
        assert_eq!(result.total_items, 3);
        assert!(result.total_kb.unwrap() >= 64);
        // 大きい順: ルート → big → small
        assert_eq!(result.items[0].depth, 0);
        assert!(result.items[1].path.ends_with("/big"));
    }
}
