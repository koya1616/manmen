//! `docker builder` の定義・実行。管理者権限は不要。
//! 削除は `prune` のみに限定し、任意の引数は受け付けない。
//! `-f` (確認なし) と `--all` (全キャッシュ) の ON/OFF だけを許可する。
//! 参照系 (`du` / `ls` / `inspect` / `version`) のうち
//! `du` / `ls` / `version` は出力をパースして構造化して返す。
//! `inspect` のみ builder 名の指定を許可し、生テキストで返す。

use serde::Serialize;

use crate::privileged;
use crate::types::CommandResult;

pub const PROGRAM: &str = "docker";

/// `docker builder prune [-f] [--all]`
pub struct DockerBuilderPrune {
    force: bool,
    all: bool,
}

impl DockerBuilderPrune {
    pub fn new(force: bool, all: bool) -> Self {
        Self { force, all }
    }

    pub fn preview(&self) -> String {
        let mut parts = vec!["docker".to_string(), "builder".to_string(), "prune".to_string()];
        if self.force {
            parts.push("-f".to_string());
        }
        if self.all {
            parts.push("--all".to_string());
        }
        parts.join(" ")
    }

    fn args(&self) -> Vec<String> {
        let mut args = vec!["builder".to_string(), "prune".to_string()];
        if self.force {
            args.push("-f".to_string());
        }
        if self.all {
            args.push("--all".to_string());
        }
        args
    }

    pub fn run(&self) -> Result<CommandResult, String> {
        let preview = self.preview();
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        privileged::execute_plain(PROGRAM, &refs, preview)
    }
}

pub fn prune_builder(force: bool, all: bool) -> Result<CommandResult, String> {
    DockerBuilderPrune::new(force, all).run()
}

/// `docker builder du` の1行。SIZE末尾の `*` は共有キャッシュを示す。
/// Frontend の `DuEntry` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct DuEntry {
    pub id: String,
    pub reclaimable: bool,
    pub shared: bool,
    pub size: String,
    pub last_accessed: String,
}

/// `docker builder du` 末尾の集計行。
/// Frontend の `DuSummary` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct DuSummary {
    pub shared: String,
    pub private: String,
    pub reclaimable: String,
    pub total: String,
}

/// `docker builder du` の返却値。Frontend の `DuSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct DuSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub entries: Vec<DuEntry>,
    pub summary: DuSummary,
    pub stderr: String,
}

/// `docker builder ls` のノード行 (`\_` マーカーを除いたもの)。
/// Frontend の `BuilderNode` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct BuilderNode {
    pub name: String,
    pub endpoint: String,
    pub status: String,
    pub buildkit: String,
    pub platforms: String,
}

/// `docker builder ls` の builder 見出しと配下ノード。
/// Frontend の `BuilderInstance` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct BuilderInstance {
    pub name: String,
    pub driver: String,
    pub is_current: bool,
    pub nodes: Vec<BuilderNode>,
}

/// `docker builder ls` の返却値。Frontend の `LsSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct LsSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub builders: Vec<BuilderInstance>,
    pub stderr: String,
}

/// `docker builder version` の1行目を割ったもの。
/// 例: `github.com/docker/buildx v0.33.0-desktop.1 7f91f03...`
/// Frontend の `BuilderVersion` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct BuilderVersion {
    pub package: String,
    pub version: String,
    pub commit: String,
}

/// `docker builder version` の返却値。Frontend の `VersionSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct VersionSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub version: BuilderVersion,
    pub raw: String,
    pub stderr: String,
}

/// `docker builder inspect` の GC ポリシー1件 (`GC Policy rule#N:`)。
/// キーは出力次第で増減するため、知っている項目以外は `extra` に残す。
/// Frontend の `GcPolicy` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct GcPolicy {
    pub name: String,
    pub all: String,
    pub filters: String,
    pub keep_duration: String,
    pub max_used_space: String,
    pub reserved_space: String,
    pub min_free_space: String,
    pub extra: Vec<String>,
}

/// `docker builder inspect` のノード1件。
/// Frontend の `InspectNode` とフィールドを一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct InspectNode {
    pub name: String,
    pub endpoint: String,
    pub status: String,
    pub buildkit: String,
    pub platforms: String,
    pub labels: Vec<String>,
    pub devices: Vec<String>,
    pub gc_policies: Vec<GcPolicy>,
}

/// `docker builder inspect [NAME]` の返却値。Frontend の `InspectSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct InspectSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub name: String,
    pub driver: String,
    pub last_activity: String,
    pub nodes: Vec<InspectNode>,
    pub stderr: String,
}

/// 参照系の素朴な実行。プレビューと引数を組み立てるだけ。
fn execute_read(subcommand: &str, extra: &[String]) -> Result<CommandResult, String> {
    let mut preview_parts = vec![
        "docker".to_string(),
        "builder".to_string(),
        subcommand.to_string(),
    ];
    preview_parts.extend(extra.iter().cloned());
    let preview = preview_parts.join(" ");
    let mut args = vec!["builder".to_string(), subcommand.to_string()];
    args.extend(extra.iter().cloned());
    let refs: Vec<&str> = args.iter().map(String::as_str).collect();
    privileged::execute_plain(PROGRAM, &refs, preview)
}

/// `docker builder du` — キャッシュ使用量を表と集計に分けて返す。
pub fn disk_usage() -> Result<DuSnapshot, String> {
    let output = execute_read("du", &[])?;
    let (entries, summary) = parse_du(&output.stdout);
    Ok(DuSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        entries,
        summary,
        stderr: output.stderr,
    })
}

/// `docker builder ls` — builder 一覧を階層のまま返す。
pub fn list_builders() -> Result<LsSnapshot, String> {
    let output = execute_read("ls", &[])?;
    let builders = parse_ls(&output.stdout);
    Ok(LsSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        builders,
        stderr: output.stderr,
    })
}

/// `docker builder inspect [NAME]` — builder 詳細を構造化して返す。空欄は現在の builder。
pub fn inspect_builder(name: String) -> Result<InspectSnapshot, String> {
    let name = name.trim().to_string();
    let output = if name.is_empty() {
        execute_read("inspect", &[])?
    } else {
        validate_builder_name(&name)?;
        execute_read("inspect", &[name])?
    };
    let (top_name, driver, last_activity, nodes) = parse_inspect(&output.stdout);
    Ok(InspectSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        name: top_name,
        driver,
        last_activity,
        nodes,
        stderr: output.stderr,
    })
}

/// `docker builder version` — バージョンを割って返す。
pub fn builder_version() -> Result<VersionSnapshot, String> {
    let output = execute_read("version", &[])?;
    let (version, raw) = parse_version(&output.stdout);
    Ok(VersionSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        version,
        raw,
        stderr: output.stderr,
    })
}

/// `docker builder inspect` 全体を先頭部とノード列に分けてパースする。
/// `Nodes:` より前が先頭部 (`Name` / `Driver` / `Last Activity`)、
/// 以降は `Name:` でノードが始まり、`Labels:` / `Devices:` / `GC Policy ...:` が副ブロックになる。
/// 知らない行・列不足は捨てる。
fn parse_inspect(stdout: &str) -> (String, String, String, Vec<InspectNode>) {
    #[derive(Clone, Copy, PartialEq)]
    enum Block {
        Scalars,
        Labels,
        Devices,
        Gc,
    }

    let mut top_name = String::new();
    let mut driver = String::new();
    let mut last_activity = String::new();
    let mut nodes: Vec<InspectNode> = Vec::new();
    let mut in_nodes = false;
    let mut block = Block::Scalars;

    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if trimmed == "Nodes:" {
            in_nodes = true;
            block = Block::Scalars;
            continue;
        }
        if trimmed == "Labels:" {
            block = Block::Labels;
            continue;
        }
        if trimmed == "Devices:" {
            block = Block::Devices;
            continue;
        }
        if trimmed.starts_with("GC Policy") && trimmed.ends_with(':') {
            let rule = trimmed
                .strip_prefix("GC Policy")
                .unwrap_or("")
                .trim()
                .trim_end_matches(':')
                .trim()
                .to_string();
            if let Some(node) = nodes.last_mut() {
                node.gc_policies.push(GcPolicy {
                    name: rule,
                    ..GcPolicy::default()
                });
            }
            block = Block::Gc;
            continue;
        }
        match block {
            Block::Labels => {
                if let Some(node) = nodes.last_mut() {
                    node.labels.push(trimmed.to_string());
                }
                continue;
            }
            Block::Devices => {
                if let Some(node) = nodes.last_mut() {
                    node.devices.push(trimmed.to_string());
                }
                continue;
            }
            Block::Gc => {
                if let Some((key, value)) = trimmed.split_once(':') {
                    if let Some(policy) =
                        nodes.last_mut().and_then(|node| node.gc_policies.last_mut())
                    {
                        match key.trim() {
                            "All" => policy.all = value.trim().to_string(),
                            "Filters" => policy.filters = value.trim().to_string(),
                            "Keep Duration" => {
                                policy.keep_duration = value.trim().to_string();
                            }
                            "Max Used Space" => {
                                policy.max_used_space = value.trim().to_string();
                            }
                            "Reserved Space" => {
                                policy.reserved_space = value.trim().to_string();
                            }
                            "Min Free Space" => {
                                policy.min_free_space = value.trim().to_string();
                            }
                            _ => policy.extra.push(trimmed.to_string()),
                        }
                        continue;
                    }
                }
                continue;
            }
            Block::Scalars => {}
        }
        let Some((key, value)) = trimmed.split_once(':') else {
            continue;
        };
        let (key, value) = (key.trim(), value.trim().to_string());
        if !in_nodes {
            match key {
                "Name" => top_name = value,
                "Driver" => driver = value,
                "Last Activity" => last_activity = value,
                _ => {}
            }
            continue;
        }
        match key {
            "Name" => nodes.push(InspectNode {
                name: value,
                ..InspectNode::default()
            }),
            "Endpoint" | "Status" | "BuildKit version" | "Platforms" => {
                if nodes.is_empty() {
                    nodes.push(InspectNode::default());
                }
                if let Some(node) = nodes.last_mut() {
                    match key {
                        "Endpoint" => node.endpoint = value,
                        "Status" => node.status = value,
                        "BuildKit version" => node.buildkit = value,
                        "Platforms" => node.platforms = value,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
    (top_name, driver, last_activity, nodes)
}

/// builder 名は `docker builder ls` に出る名前のみ。オプションに見えないよう制限する。
fn validate_builder_name(name: &str) -> Result<(), String> {
    let ok = (1..=64).contains(&name.len())
        && !name.starts_with('-')
        && name
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | '-'));
    if ok {
        Ok(())
    } else {
        Err(format!("builder 名が不正です: {name}"))
    }
}

/// 表部 (`ID RECLAIMABLE SIZE LAST ACCESSED`) と集計部 (`Shared:` 等) に分けてパースする。
/// 見出しのない行・列不足の行は捨てる。
fn parse_du(stdout: &str) -> (Vec<DuEntry>, DuSummary) {
    let mut entries = Vec::new();
    let mut summary = DuSummary::default();
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(':') {
            match key.trim() {
                "Shared" => summary.shared = value.trim().to_string(),
                "Private" => summary.private = value.trim().to_string(),
                "Reclaimable" => summary.reclaimable = value.trim().to_string(),
                "Total" => summary.total = value.trim().to_string(),
                _ => {}
            }
            continue;
        }
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        if tokens.len() < 4 {
            continue;
        }
        if tokens[0] == "ID" && tokens.contains(&"RECLAIMABLE") {
            continue;
        }
        let (size, shared) = match tokens[2].strip_suffix('*') {
            Some(bare) => (bare.to_string(), true),
            None => (tokens[2].to_string(), false),
        };
        entries.push(DuEntry {
            id: tokens[0].to_string(),
            reclaimable: tokens[1] == "true",
            shared,
            size,
            last_accessed: tokens[3..].join(" "),
        });
    }
    (entries, summary)
}

/// 見出し (`NAME/NODE ...`) を除き、builder 行と `\_` ノード行に分けて階層化する。
/// 列不足の行は捨てる。
fn parse_ls(stdout: &str) -> Vec<BuilderInstance> {
    let mut builders: Vec<BuilderInstance> = Vec::new();
    for line in stdout.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let head = line.trim_start();
        if head.starts_with("NAME") {
            continue;
        }
        if head.starts_with('\\') {
            let tokens: Vec<&str> = head
                .split_whitespace()
                .filter(|token| *token != "\\_")
                .collect();
            if tokens.len() < 4 {
                continue;
            }
            let Some(current) = builders.last_mut() else {
                continue;
            };
            current.nodes.push(BuilderNode {
                name: tokens[0].to_string(),
                endpoint: tokens.get(1).unwrap_or(&"").to_string(),
                status: tokens.get(2).unwrap_or(&"").to_string(),
                buildkit: tokens.get(3).unwrap_or(&"").to_string(),
                platforms: tokens.get(4..).unwrap_or(&[]).join(" "),
            });
        } else {
            let tokens: Vec<&str> = head.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }
            let (name, is_current) = match tokens[0].strip_suffix('*') {
                Some(bare) => (bare.to_string(), true),
                None => (tokens[0].to_string(), false),
            };
            builders.push(BuilderInstance {
                name,
                driver: tokens.get(1).unwrap_or(&"").to_string(),
                is_current,
                nodes: Vec::new(),
            });
        }
    }
    builders
}

/// 1行目を `package version commit` に割る。形が違えば空のまま `raw` に残す。
fn parse_version(stdout: &str) -> (BuilderVersion, String) {
    let raw = stdout.lines().map(str::trim).find(|line| !line.is_empty());
    let Some(first) = raw else {
        return (BuilderVersion::default(), String::new());
    };
    let tokens: Vec<&str> = first.split_whitespace().collect();
    let version = BuilderVersion {
        package: tokens.first().unwrap_or(&"").to_string(),
        version: tokens.get(1).unwrap_or(&"").to_string(),
        commit: tokens.get(2).unwrap_or(&"").to_string(),
    };
    (version, first.to_string())
}

#[cfg(test)]
mod tests {
    use super::{
        parse_du, parse_inspect, parse_ls, parse_version, validate_builder_name,
        DockerBuilderPrune,
    };

    const DU_SAMPLE: &str = "ID                          RECLAIMABLE   SIZE       LAST ACCESSED\n\
         63n3t680qe8dlu302jil5hsut   true          0B         8 months ago\n\
         cslnnwe06xtf850oqdajmsypa   true          1.603kB*   27 minutes ago\n\
         \n\
         Shared:\t\t1.234GB\n\
         Private:\t0B\n\
         Reclaimable:\t1.234GB\n\
         Total:\t\t1.234GB\n";

    const LS_SAMPLE: &str = "NAME/NODE           DRIVER/ENDPOINT     STATUS    BUILDKIT   PLATFORMS\n\
         default             docker\n\
          \\_ default          \\_ default         running   v0.29.0    linux/amd64 (+2), linux/arm64\n\
         desktop-linux*      docker\n\
          \\_ desktop-linux    \\_ desktop-linux   running   v0.29.0    linux/arm64, linux/amd64\n";

    const INSPECT_SAMPLE: &str = "Name:          desktop-linux\n\
         Driver:        docker\n\
         Last Activity: 2026-10-01 13:04:26 +0000 UTC\n\
         \n\
         Nodes:\n\
         Name:             desktop-linux\n\
         Endpoint:         desktop-linux\n\
         Status:           running\n\
         BuildKit version: v0.29.0\n\
         Platforms:        linux/arm64, linux/amd64\n\
         Labels:\n\
          org.mobyproject.buildkit.worker.moby.host-gateway-ip: 192.168.65.254\n\
         Devices:\n\
          Name:                  docker.com/gpu=webgpu\n\
          Automatically allowed: false\n\
         GC Policy rule#0:\n\
          All:            false\n\
          Filters:        type==source.local,type==exec.cachemount\n\
          Keep Duration:  48h0m0s\n\
          Max Used Space: 2.764GiB\n\
         GC Policy rule#1:\n\
          All:            true\n\
          Reserved Space: 20GiB\n";

    #[test]
    fn previews_force_flag() {
        assert_eq!(
            DockerBuilderPrune::new(true, false).preview(),
            "docker builder prune -f"
        );
    }

    #[test]
    fn previews_all_options() {
        assert_eq!(
            DockerBuilderPrune::new(false, false).preview(),
            "docker builder prune"
        );
        assert_eq!(
            DockerBuilderPrune::new(true, true).preview(),
            "docker builder prune -f --all"
        );
        assert_eq!(
            DockerBuilderPrune::new(false, true).preview(),
            "docker builder prune --all"
        );
    }

    #[test]
    fn accepts_valid_builder_names() {
        assert!(validate_builder_name("default").is_ok());
        assert!(validate_builder_name("desktop-linux").is_ok());
        assert!(validate_builder_name("my.builder_1").is_ok());
    }

    #[test]
    fn rejects_option_like_builder_names() {
        assert!(validate_builder_name("").is_err());
        assert!(validate_builder_name("-f").is_err());
        assert!(validate_builder_name("--all").is_err());
        assert!(validate_builder_name("a;rm -rf /").is_err());
        assert!(validate_builder_name("with space").is_err());
    }

    #[test]
    fn parses_du_entries_and_summary() {
        let (entries, summary) = parse_du(DU_SAMPLE);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].id, "63n3t680qe8dlu302jil5hsut");
        assert!(entries[0].reclaimable);
        assert!(!entries[0].shared);
        assert_eq!(entries[0].size, "0B");
        assert_eq!(entries[0].last_accessed, "8 months ago");
        assert!(entries[1].shared);
        assert_eq!(entries[1].size, "1.603kB");
        assert_eq!(entries[1].last_accessed, "27 minutes ago");
        assert_eq!(summary.shared, "1.234GB");
        assert_eq!(summary.private, "0B");
        assert_eq!(summary.reclaimable, "1.234GB");
        assert_eq!(summary.total, "1.234GB");
    }

    #[test]
    fn parses_ls_hierarchy() {
        let builders = parse_ls(LS_SAMPLE);
        assert_eq!(builders.len(), 2);
        assert_eq!(builders[0].name, "default");
        assert!(!builders[0].is_current);
        assert_eq!(builders[0].nodes.len(), 1);
        assert_eq!(builders[0].nodes[0].status, "running");
        assert_eq!(builders[0].nodes[0].buildkit, "v0.29.0");
        assert!(builders[0].nodes[0].platforms.contains("linux/amd64"));
        assert_eq!(builders[1].name, "desktop-linux");
        assert!(builders[1].is_current);
        assert_eq!(builders[1].nodes[0].name, "desktop-linux");
    }

    #[test]
    fn parses_version_line() {
        let (version, raw) = parse_version(
            "github.com/docker/buildx v0.33.0-desktop.1 7f91f038ac14cbf5c4b2a6b76470860814424da1\n",
        );
        assert_eq!(version.package, "github.com/docker/buildx");
        assert_eq!(version.version, "v0.33.0-desktop.1");
        assert_eq!(
            version.commit,
            "7f91f038ac14cbf5c4b2a6b76470860814424da1"
        );
        assert!(!raw.is_empty());
        let (empty, raw) = parse_version("\n");
        assert_eq!(raw, "");
        assert_eq!(empty.package, "");
    }

    #[test]
    fn parses_inspect_sections() {
        let (name, driver, last_activity, nodes) = parse_inspect(INSPECT_SAMPLE);
        assert_eq!(name, "desktop-linux");
        assert_eq!(driver, "docker");
        assert_eq!(last_activity, "2026-10-01 13:04:26 +0000 UTC");
        assert_eq!(nodes.len(), 1);
        let node = &nodes[0];
        assert_eq!(node.name, "desktop-linux");
        assert_eq!(node.endpoint, "desktop-linux");
        assert_eq!(node.status, "running");
        assert_eq!(node.buildkit, "v0.29.0");
        assert!(node.platforms.contains("linux/arm64"));
        assert_eq!(node.labels.len(), 1);
        assert!(node.labels[0].contains("host-gateway-ip"));
        assert_eq!(node.devices.len(), 2);
        assert_eq!(node.gc_policies.len(), 2);
        assert_eq!(node.gc_policies[0].name, "rule#0");
        assert_eq!(node.gc_policies[0].all, "false");
        assert!(node.gc_policies[0].filters.contains("source.local"));
        assert_eq!(node.gc_policies[0].keep_duration, "48h0m0s");
        assert_eq!(node.gc_policies[0].max_used_space, "2.764GiB");
        assert_eq!(node.gc_policies[1].name, "rule#1");
        assert_eq!(node.gc_policies[1].all, "true");
        assert_eq!(node.gc_policies[1].reserved_space, "20GiB");
    }
}
