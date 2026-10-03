//! Tauri IPC エンドポイント。薄く保つこと。
//! 実処理は各コマンドモジュール (`pmset.rs`・`manpage.rs` 等) と共通基盤 (`privileged.rs`) に置く。

use crate::dig::DigSnapshot;
use crate::lsof::LsofSnapshot;
use crate::manpage::ManpageDocument;
use crate::pmset::PmsetState;
use crate::ps::PsSnapshot;
use crate::ssh::SshSnapshot;
use crate::top::TopSnapshot;
use crate::types::CommandResult;

#[tauri::command]
pub fn get_pmset() -> Result<PmsetState, String> {
    crate::pmset::current()
}

#[tauri::command]
pub fn set_pmset(scope: String, setting: String, value: String) -> Result<CommandResult, String> {
    crate::pmset::apply(scope, setting, value)
}

#[tauri::command]
pub fn get_manpage(topic: String) -> Result<ManpageDocument, String> {
    crate::manpage::Manpage::new(&topic).and_then(|cmd| cmd.run())
}

#[tauri::command]
pub fn get_dig(query: crate::dig::DigQuery) -> Result<DigSnapshot, String> {
    crate::dig::get_snapshot(query)
}

#[tauri::command]
pub fn get_ssh(query: crate::ssh::SshQuery) -> Result<SshSnapshot, String> {
    crate::ssh::get_snapshot(query)
}

#[tauri::command]
pub fn start_ssh_tunnel(query: crate::ssh::TunnelQuery) -> Result<crate::ssh::TunnelStarted, String> {
    crate::ssh::start_tunnel(query)
}

#[tauri::command]
pub fn stop_ssh_tunnel(id: String) -> Result<(), String> {
    crate::ssh::stop_tunnel(&id)
}

#[tauri::command]
pub fn list_ssh_tunnels() -> Vec<crate::ssh::TunnelInfo> {
    crate::ssh::list_tunnels()
}

#[tauri::command]
pub fn list_ssh_hosts() -> Result<Vec<crate::ssh::SshKnownHost>, String> {
    crate::ssh::list_known_hosts()
}

#[tauri::command]
pub fn get_ps(query: crate::ps::PsQuery) -> Result<PsSnapshot, String> {
    crate::ps::get_snapshot(query)
}

#[tauri::command]
pub fn get_lsof(query: crate::lsof::LsofQuery) -> Result<LsofSnapshot, String> {
    crate::lsof::get_snapshot(query)
}

#[tauri::command]
pub fn get_top(query: crate::top::TopQuery) -> Result<TopSnapshot, String> {
    crate::top::get_snapshot(query)
}

#[tauri::command]
pub fn prune_docker_builder(force: bool, all: bool) -> Result<CommandResult, String> {
    crate::docker::prune_builder(force, all)
}

#[tauri::command]
pub fn docker_builder_du() -> Result<crate::docker::DuSnapshot, String> {
    crate::docker::disk_usage()
}

#[tauri::command]
pub fn docker_builder_ls() -> Result<crate::docker::LsSnapshot, String> {
    crate::docker::list_builders()
}

#[tauri::command]
pub fn docker_builder_inspect(name: String) -> Result<crate::docker::InspectSnapshot, String> {
    crate::docker::inspect_builder(name)
}

#[tauri::command]
pub fn docker_builder_version() -> Result<crate::docker::VersionSnapshot, String> {
    crate::docker::builder_version()
}

#[tauri::command]
pub fn docker_containers() -> Result<crate::docker_read::ContainerSnapshot, String> {
    crate::docker_read::list_containers()
}

#[tauri::command]
pub fn docker_images() -> Result<crate::docker_read::ImageSnapshot, String> {
    crate::docker_read::list_images()
}

#[tauri::command]
pub fn docker_networks() -> Result<crate::docker_read::NetworkSnapshot, String> {
    crate::docker_read::list_networks()
}

#[tauri::command]
pub fn docker_volumes() -> Result<crate::docker_read::VolumeSnapshot, String> {
    crate::docker_read::list_volumes()
}

#[tauri::command]
pub fn docker_system_df() -> Result<crate::docker_read::SystemDfSnapshot, String> {
    crate::docker_read::system_df()
}

#[tauri::command]
pub fn docker_system_info() -> Result<crate::docker_read::SystemInfoSnapshot, String> {
    crate::docker_read::system_info()
}

#[tauri::command]
pub fn get_whoami() -> Result<CommandResult, String> {
    crate::whoami::current()
}

#[tauri::command]
pub fn get_who() -> Result<crate::who::WhoSnapshot, String> {
    crate::who::current()
}

#[tauri::command]
pub fn get_w() -> Result<crate::w::WSnapshot, String> {
    crate::w::current()
}

#[tauri::command]
pub fn get_system_profiler(
    query: crate::system_profiler::SystemProfilerQuery,
) -> Result<crate::system_profiler::SystemProfilerSnapshot, String> {
    crate::system_profiler::get_snapshot(query)
}

#[tauri::command]
pub fn set_remember(enabled: bool) {
    crate::privileged::set_remember(enabled);
}
