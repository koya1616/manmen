//! `df` の定義・実行・出力パース。管理者権限は不要。
//! 表示の単位変換は Frontend で行うため、常に `-k` (KiB)・`-Y` (種別)・`-i` (inode) で取得する。
//! 古い統計を表示する `-n` や、単位指定 (`-h`/`-H`/`-m`/`-g`/`-b`/`-P`) は扱わない。

use serde::{Deserialize, Serialize};

use crate::common::parse::nth_field_start;
use crate::common::validate::resolve_existing_path;
use crate::privileged;

pub const FS_TYPES: &[&str] = &["apfs", "hfs", "msdos", "exfat", "smbfs", "nfs"];

/// Frontend の `getDf` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DfQuery {
    /// 空なら全ファイルシステム
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub local: bool,
    /// 空なら種別で絞り込まない
    #[serde(default)]
    pub fs_type: String,
}

/// 1ファイルシステム。Frontend の `DfRow` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DfRow {
    pub filesystem: String,
    pub fs_type: String,
    pub size_kb: u64,
    pub used_kb: u64,
    pub avail_kb: u64,
    pub capacity_percent: u32,
    pub inodes_used: u64,
    pub inodes_free: u64,
    pub inode_percent: u32,
    pub mount: String,
}

/// `get_df` の返却値。Frontend の `DfSnapshot` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct DfSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub rows: Vec<DfRow>,
    pub stderr: String,
}

pub struct Df {
    args: Vec<String>,
}

impl Df {
    pub fn new(query: DfQuery) -> Result<Self, String> {
        let mut args: Vec<String> = vec!["-k".to_string(), "-Y".to_string(), "-i".to_string()];
        if query.all {
            args.push("-a".to_string());
        }
        if query.local {
            args.push("-l".to_string());
        }
        let fs_type = query.fs_type.trim();
        if !fs_type.is_empty() {
            if !FS_TYPES.contains(&fs_type) {
                return Err(format!("ファイルシステム種別が不正です: {fs_type}"));
            }
            args.push("-T".to_string());
            args.push(fs_type.to_string());
        }
        if !query.path.trim().is_empty() {
            args.push(resolve_existing_path(&query.path)?);
        }
        Ok(Self { args })
    }

    pub fn preview(&self) -> String {
        format!("df {}", self.args.join(" "))
    }

    pub fn run(&self) -> Result<DfSnapshot, String> {
        let preview = self.preview();
        let refs: Vec<&str> = self.args.iter().map(String::as_str).collect();
        let output = privileged::execute_plain("/bin/df", &refs, preview.clone())?;
        Ok(DfSnapshot {
            success: output.success,
            exit_code: output.exit_code,
            command: preview,
            rows: parse_rows(&output.stdout),
            stderr: output.stderr.trim().to_string(),
        })
    }
}

pub fn get_snapshot(query: DfQuery) -> Result<DfSnapshot, String> {
    Df::new(query).and_then(|cmd| cmd.run())
}

/// `-k -Y -i` の行をパースする。
/// 先頭のファイルシステム名 (`map auto_home`) と末尾のマウント先 (`/Volumes/My Disk`) は
/// 空白を含みうるため、間にある数値列の並びを手がかりに位置を決める。
fn parse_rows(stdout: &str) -> Vec<DfRow> {
    stdout.lines().skip(1).filter_map(parse_row).collect()
}

fn parse_row(line: &str) -> Option<DfRow> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    // tokens[j] が種別、その後に 1K-blocks / Used / Avail / Capacity / iused / ifree / %iused / マウント先
    let j = (1..tokens.len().saturating_sub(8)).find(|&j| {
        is_num(tokens[j + 1])
            && is_num(tokens[j + 2])
            && is_num(tokens[j + 3])
            && is_percent(tokens[j + 4])
            && is_num(tokens[j + 5])
            && is_num(tokens[j + 6])
            && is_percent(tokens[j + 7])
    })?;
    Some(DfRow {
        filesystem: tokens[..j].join(" "),
        fs_type: tokens[j].to_string(),
        size_kb: tokens[j + 1].parse().ok()?,
        used_kb: tokens[j + 2].parse().ok()?,
        avail_kb: tokens[j + 3].parse().ok()?,
        capacity_percent: percent(tokens[j + 4])?,
        inodes_used: tokens[j + 5].parse().ok()?,
        inodes_free: tokens[j + 6].parse().ok()?,
        inode_percent: percent(tokens[j + 7])?,
        // マウント先は空白を含むため原文から取り直す (1始まりで j+9 番目)
        mount: nth_field_start(line, j + 9)?.to_string(),
    })
}

fn is_num(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|c| c.is_ascii_digit())
}

fn is_percent(token: &str) -> bool {
    token.strip_suffix('%').map(is_num).unwrap_or(false)
}

fn percent(token: &str) -> Option<u32> {
    token.strip_suffix('%')?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{Df, DfQuery};

    const OUTPUT: &str = "\
Filesystem      Type   1024-blocks      Used Available Capacity iused      ifree %iused  Mounted on
/dev/disk3s1s1  apfs     482797652  12345632  95471852    12%  458732  954718520    0%   /
map auto_home   autofs           0         0         0   100%       0          0  100%   /System/Volumes/Data/home
/dev/disk10s1   hfs         589468    588316      1152   100%      10       4294  1%   /Volumes/Orca 1.4.217-arm64
";

    fn base_query() -> DfQuery {
        DfQuery {
            path: String::new(),
            all: false,
            local: false,
            fs_type: String::new(),
        }
    }

    #[test]
    fn previews_queries() {
        assert_eq!(Df::new(base_query()).unwrap().preview(), "df -k -Y -i");

        let mut query = base_query();
        query.all = true;
        query.local = true;
        query.fs_type = "apfs".to_string();
        query.path = "/".to_string();
        assert_eq!(Df::new(query).unwrap().preview(), "df -k -Y -i -a -l -T apfs /");
    }

    #[test]
    fn rejects_bad_input() {
        let mut query = base_query();
        query.fs_type = "ext4".to_string();
        assert!(Df::new(query).is_err());

        for path in ["-h", "relative/path", "/no/such/path/manmen"] {
            let mut query = base_query();
            query.path = path.to_string();
            assert!(Df::new(query).is_err(), "{path}");
        }
    }

    #[test]
    fn parses_rows_with_spaces() {
        let rows = super::parse_rows(OUTPUT);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].filesystem, "/dev/disk3s1s1");
        assert_eq!(rows[0].fs_type, "apfs");
        assert_eq!(rows[0].size_kb, 482797652);
        assert_eq!(rows[0].capacity_percent, 12);
        assert_eq!(rows[0].mount, "/");
        assert_eq!(rows[1].filesystem, "map auto_home");
        assert_eq!(rows[1].fs_type, "autofs");
        assert_eq!(rows[2].mount, "/Volumes/Orca 1.4.217-arm64");
        assert_eq!(rows[2].inodes_free, 4294);
    }

    #[test]
    fn runs_df_root() {
        let mut query = base_query();
        query.path = "/".to_string();
        let result = super::get_snapshot(query).expect("df");
        assert!(result.success, "stderr: {}", result.stderr);
        assert_eq!(result.rows.len(), 1);
        assert_eq!(result.rows[0].mount, "/");
    }
}
