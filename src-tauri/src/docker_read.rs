//! `docker` 参照系 (read-only) の定義・実行。管理者権限は不要。
//! すべて `--format json` の1行1JSON出力をパースして構造化して返す。
//! 任意の引数・フィルタ・テンプレートは受け付けない (固定引数のみ)。

use serde::Serialize;

use crate::privileged;
use crate::types::CommandResult;

pub const PROGRAM: &str = "docker";

fn get_str(v: &serde_json::Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or("").to_string()
}

fn get_int(v: &serde_json::Value, key: &str) -> i64 {
    v.get(key)
        .and_then(|x| x.as_i64())
        .or_else(|| v.get(key).and_then(|x| x.as_str()).and_then(|s| s.parse().ok()))
        .unwrap_or(0)
}

/// 固定引数で実行し、素の結果を返す。
fn execute_fixed(preview_parts: &[&str], args: &[&str]) -> Result<CommandResult, String> {
    let preview = preview_parts.join(" ");
    privileged::execute_plain(PROGRAM, args, preview)
}

/// 行区切りJSONの各行を `serde_json::Value` にする。壊れた行は捨てる。
fn parse_json_lines(stdout: &str) -> Vec<serde_json::Value> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect()
}

// ---------- containers ----------

/// Frontend の `DockerContainerEntry` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ContainerEntry {
    pub id: String,
    pub names: String,
    pub image: String,
    pub command: String,
    pub created: String,
    pub status: String,
    pub state: String,
    pub ports: String,
}

/// Frontend の `ContainerSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct ContainerSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub containers: Vec<ContainerEntry>,
    pub stderr: String,
}

/// `docker container ls -a --format json` — 停止中も含めて一覧する。
pub fn list_containers() -> Result<ContainerSnapshot, String> {
    let output = execute_fixed(
        &["docker", "container", "ls", "-a", "--format", "json"],
        &["container", "ls", "-a", "--format", "json"],
    )?;
    let containers = parse_json_lines(&output.stdout)
        .iter()
        .map(|v| ContainerEntry {
            id: get_str(v, "ID"),
            names: get_str(v, "Names"),
            image: get_str(v, "Image"),
            command: get_str(v, "Command"),
            created: get_str(v, "RunningFor"),
            status: get_str(v, "Status"),
            state: get_str(v, "State"),
            ports: get_str(v, "Ports"),
        })
        .filter(|e| !e.id.is_empty())
        .collect();
    Ok(ContainerSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        containers,
        stderr: output.stderr,
    })
}

// ---------- images ----------

/// Frontend の `DockerImageEntry` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct ImageEntry {
    pub repository: String,
    pub tag: String,
    pub id: String,
    pub created_since: String,
    pub size: String,
    pub containers: String,
}

/// Frontend の `ImageSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct ImageSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub images: Vec<ImageEntry>,
    pub stderr: String,
}

/// `docker image ls --format json` — 中間・dangling は既定で隠す。
pub fn list_images() -> Result<ImageSnapshot, String> {
    let output = execute_fixed(
        &["docker", "image", "ls", "--format", "json"],
        &["image", "ls", "--format", "json"],
    )?;
    let images = parse_json_lines(&output.stdout)
        .iter()
        .map(|v| ImageEntry {
            repository: get_str(v, "Repository"),
            tag: get_str(v, "Tag"),
            id: get_str(v, "ID"),
            created_since: get_str(v, "CreatedSince"),
            size: get_str(v, "Size"),
            containers: get_str(v, "Containers"),
        })
        .filter(|e| !e.id.is_empty())
        .collect();
    Ok(ImageSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        images,
        stderr: output.stderr,
    })
}

// ---------- networks ----------

/// Frontend の `DockerNetworkEntry` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct NetworkEntry {
    pub id: String,
    pub name: String,
    pub driver: String,
    pub scope: String,
}

/// Frontend の `NetworkSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct NetworkSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub networks: Vec<NetworkEntry>,
    pub stderr: String,
}

/// `docker network ls --format json`
pub fn list_networks() -> Result<NetworkSnapshot, String> {
    let output = execute_fixed(
        &["docker", "network", "ls", "--format", "json"],
        &["network", "ls", "--format", "json"],
    )?;
    let networks = parse_json_lines(&output.stdout)
        .iter()
        .map(|v| NetworkEntry {
            id: get_str(v, "ID"),
            name: get_str(v, "Name"),
            driver: get_str(v, "Driver"),
            scope: get_str(v, "Scope"),
        })
        .filter(|e| !e.id.is_empty())
        .collect();
    Ok(NetworkSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        networks,
        stderr: output.stderr,
    })
}

// ---------- volumes ----------

/// Frontend の `DockerVolumeEntry` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct VolumeEntry {
    pub driver: String,
    pub name: String,
    pub scope: String,
}

/// Frontend の `VolumeSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct VolumeSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub volumes: Vec<VolumeEntry>,
    pub stderr: String,
}

/// `docker volume ls --format json`
pub fn list_volumes() -> Result<VolumeSnapshot, String> {
    let output = execute_fixed(
        &["docker", "volume", "ls", "--format", "json"],
        &["volume", "ls", "--format", "json"],
    )?;
    let volumes = parse_json_lines(&output.stdout)
        .iter()
        .map(|v| VolumeEntry {
            driver: get_str(v, "Driver"),
            name: get_str(v, "Name"),
            scope: get_str(v, "Scope"),
        })
        .filter(|e| !e.name.is_empty())
        .collect();
    Ok(VolumeSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        volumes,
        stderr: output.stderr,
    })
}

// ---------- system df ----------

/// Frontend の `SystemDfEntry` と一致させること。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct SystemDfEntry {
    pub dtype: String,
    pub total: String,
    pub active: String,
    pub size: String,
    pub reclaimable: String,
}

/// Frontend の `SystemDfSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct SystemDfSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub entries: Vec<SystemDfEntry>,
    pub stderr: String,
}

/// `docker system df --format json` — verbose なしの集計のみ。
pub fn system_df() -> Result<SystemDfSnapshot, String> {
    let output = execute_fixed(
        &["docker", "system", "df", "--format", "json"],
        &["system", "df", "--format", "json"],
    )?;
    let entries = parse_json_lines(&output.stdout)
        .iter()
        .map(|v| SystemDfEntry {
            dtype: get_str(v, "Type"),
            total: get_str(v, "TotalCount"),
            active: get_str(v, "Active"),
            size: get_str(v, "Size"),
            reclaimable: get_str(v, "Reclaimable"),
        })
        .filter(|e| !e.dtype.is_empty())
        .collect();
    Ok(SystemDfSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        entries,
        stderr: output.stderr,
    })
}

// ---------- system info ----------

/// Frontend の `SystemInfoData` と一致させること。表に出す最小限だけ抜く。
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct SystemInfoData {
    pub containers: i64,
    pub containers_running: i64,
    pub containers_stopped: i64,
    pub images: i64,
    pub driver: String,
    pub server_version: String,
    pub operating_system: String,
    pub architecture: String,
    pub ncpu: i64,
    pub mem_total: i64,
    pub kernel_version: String,
}

/// Frontend の `SystemInfoSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct SystemInfoSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub info: SystemInfoData,
    pub stderr: String,
}

/// `docker info --format json` — 単一JSONを抜粋する。
pub fn system_info() -> Result<SystemInfoSnapshot, String> {
    let output = execute_fixed(
        &["docker", "info", "--format", "json"],
        &["info", "--format", "json"],
    )?;
    let v: serde_json::Value =
        serde_json::from_str(output.stdout.trim()).unwrap_or(serde_json::Value::Null);
    let info = SystemInfoData {
        containers: get_int(&v, "Containers"),
        containers_running: get_int(&v, "ContainersRunning"),
        containers_stopped: get_int(&v, "ContainersStopped"),
        images: get_int(&v, "Images"),
        driver: get_str(&v, "Driver"),
        server_version: get_str(&v, "ServerVersion"),
        operating_system: get_str(&v, "OperatingSystem"),
        architecture: get_str(&v, "Architecture"),
        ncpu: get_int(&v, "NCPU"),
        mem_total: get_int(&v, "MemTotal"),
        kernel_version: get_str(&v, "KernelVersion"),
    };
    Ok(SystemInfoSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: output.command,
        info,
        stderr: output.stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        parse_json_lines, SystemInfoData,
    };

    const CONTAINER_LINE: &str = r#"{"ID":"26395160de1f","Image":"backend-api","Names":"chonps-backend","RunningFor":"30 hours ago","State":"running","Status":"Up 30 hours","Ports":"0.0.0.0:4000->4000/tcp"}"#;
    const IMAGE_LINE: &str = r#"{"Repository":"alpine","Tag":"latest","ID":"33bee74c45f3","CreatedSince":"2 weeks ago","Size":"8.66MB","Containers":"0"}"#;

    #[test]
    fn parses_container_json_line() {
        let vals = parse_json_lines(CONTAINER_LINE);
        assert_eq!(vals.len(), 1);
        assert_eq!(vals[0].get("ID").and_then(|x| x.as_str()), Some("26395160de1f"));
        assert_eq!(vals[0].get("State").and_then(|x| x.as_str()), Some("running"));
    }

    #[test]
    fn skips_broken_lines() {
        let vals = parse_json_lines(&format!("{CONTAINER_LINE}\nnot json\n{IMAGE_LINE}\n"));
        assert_eq!(vals.len(), 2);
    }

    #[test]
    fn parses_info_subset() {
        let v: serde_json::Value = serde_json::from_str(
            r#"{"Containers":2,"ContainersRunning":2,"ContainersStopped":0,"Images":3,"Driver":"overlay2","ServerVersion":"29.4.1","Architecture":"aarch64","NCPU":4,"MemTotal":8217903104}"#,
        )
        .unwrap();
        let info = SystemInfoData {
            containers: super::get_int(&v, "Containers"),
            containers_running: super::get_int(&v, "ContainersRunning"),
            images: super::get_int(&v, "Images"),
            driver: super::get_str(&v, "Driver"),
            server_version: super::get_str(&v, "ServerVersion"),
            ..SystemInfoData::default()
        };
        assert_eq!(info.containers, 2);
        assert_eq!(info.driver, "overlay2");
        assert_eq!(info.server_version, "29.4.1");
    }
}
