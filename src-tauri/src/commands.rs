//! Tauri IPC エンドポイント。薄く保つこと。
//! 実処理は各コマンドモジュール (`pmset.rs`・`manpage.rs` 等) と共通基盤 (`privileged.rs`) に置く。
//! 属性なしの `#[tauri::command]` はメインスレッドで動き、実行中ウィンドウが固まるため、
//! 新しいエンドポイントも含め必ず `#[tauri::command(async)]` を付けること。

use crate::dig::DigSnapshot;
use crate::lsof::LsofSnapshot;
use crate::manpage::ManpageDocument;
use crate::pmset::PmsetState;
use crate::ps::PsSnapshot;
use crate::ssh::SshSnapshot;
use crate::top::TopSnapshot;
use crate::types::CommandResult;

#[tauri::command(async)]
pub fn get_pmset() -> Result<PmsetState, String> {
    crate::pmset::current()
}

#[tauri::command(async)]
pub fn set_pmset(scope: String, setting: String, value: String) -> Result<CommandResult, String> {
    crate::pmset::apply(scope, setting, value)
}

#[tauri::command(async)]
pub fn get_manpage(topic: String) -> Result<ManpageDocument, String> {
    crate::manpage::Manpage::new(&topic).and_then(|cmd| cmd.run())
}

#[tauri::command(async)]
pub fn get_dig(query: crate::dig::DigQuery) -> Result<DigSnapshot, String> {
    crate::dig::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_ping(query: crate::ping::PingQuery) -> Result<crate::ping::PingSnapshot, String> {
    crate::ping::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_curl(query: crate::curl::CurlQuery) -> Result<crate::curl::CurlSnapshot, String> {
    crate::curl::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_kill_targets(pids: String) -> Result<Vec<crate::kill::KillTarget>, String> {
    crate::kill::get_targets(pids)
}

#[tauri::command(async)]
pub fn send_kill(query: crate::kill::KillQuery) -> Result<crate::kill::KillSnapshot, String> {
    crate::kill::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_traceroute(
    query: crate::traceroute::TracerouteQuery,
) -> Result<crate::traceroute::TracerouteSnapshot, String> {
    crate::traceroute::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_df(query: crate::df::DfQuery) -> Result<crate::df::DfSnapshot, String> {
    crate::df::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_du(query: crate::du::DuQuery) -> Result<crate::du::DuSnapshot, String> {
    crate::du::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_ifconfig(
    query: crate::ifconfig::IfconfigQuery,
) -> Result<crate::ifconfig::IfconfigSnapshot, String> {
    crate::ifconfig::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_networksetup(
    query: crate::networksetup::NetworksetupQuery,
) -> Result<crate::networksetup::NetworksetupResult, String> {
    crate::networksetup::get_snapshot(query)
}

#[tauri::command(async)]
pub fn list_network_services() -> Result<Vec<String>, String> {
    crate::networksetup::list_service_names()
}

#[tauri::command(async)]
pub fn get_ssh(query: crate::ssh::SshQuery) -> Result<SshSnapshot, String> {
    crate::ssh::get_snapshot(query)
}

#[tauri::command(async)]
pub fn start_ssh_tunnel(query: crate::ssh::TunnelQuery) -> Result<crate::ssh::TunnelStarted, String> {
    crate::ssh::start_tunnel(query)
}

#[tauri::command(async)]
pub fn stop_ssh_tunnel(id: String) -> Result<(), String> {
    crate::ssh::stop_tunnel(&id)
}

#[tauri::command(async)]
pub fn list_ssh_tunnels() -> Vec<crate::ssh::TunnelInfo> {
    crate::ssh::list_tunnels()
}

#[tauri::command(async)]
pub fn list_ssh_hosts() -> Result<Vec<crate::ssh::SshKnownHost>, String> {
    crate::ssh::list_known_hosts()
}

#[tauri::command(async)]
pub fn get_ps(query: crate::ps::PsQuery) -> Result<PsSnapshot, String> {
    crate::ps::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_lsof(query: crate::lsof::LsofQuery) -> Result<LsofSnapshot, String> {
    crate::lsof::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_top(query: crate::top::TopQuery) -> Result<TopSnapshot, String> {
    crate::top::get_snapshot(query)
}

#[tauri::command(async)]
pub fn prune_docker_builder(force: bool, all: bool) -> Result<CommandResult, String> {
    crate::docker::prune_builder(force, all)
}

#[tauri::command(async)]
pub fn docker_builder_du() -> Result<crate::docker::DuSnapshot, String> {
    crate::docker::disk_usage()
}

#[tauri::command(async)]
pub fn docker_builder_ls() -> Result<crate::docker::LsSnapshot, String> {
    crate::docker::list_builders()
}

#[tauri::command(async)]
pub fn docker_builder_inspect(name: String) -> Result<crate::docker::InspectSnapshot, String> {
    crate::docker::inspect_builder(name)
}

#[tauri::command(async)]
pub fn docker_builder_version() -> Result<crate::docker::VersionSnapshot, String> {
    crate::docker::builder_version()
}

#[tauri::command(async)]
pub fn docker_containers() -> Result<crate::docker_read::ContainerSnapshot, String> {
    crate::docker_read::list_containers()
}

#[tauri::command(async)]
pub fn docker_images() -> Result<crate::docker_read::ImageSnapshot, String> {
    crate::docker_read::list_images()
}

#[tauri::command(async)]
pub fn docker_networks() -> Result<crate::docker_read::NetworkSnapshot, String> {
    crate::docker_read::list_networks()
}

#[tauri::command(async)]
pub fn docker_volumes() -> Result<crate::docker_read::VolumeSnapshot, String> {
    crate::docker_read::list_volumes()
}

#[tauri::command(async)]
pub fn docker_system_df() -> Result<crate::docker_read::SystemDfSnapshot, String> {
    crate::docker_read::system_df()
}

#[tauri::command(async)]
pub fn docker_system_info() -> Result<crate::docker_read::SystemInfoSnapshot, String> {
    crate::docker_read::system_info()
}

#[tauri::command(async)]
pub fn get_whoami() -> Result<CommandResult, String> {
    crate::whoami::current()
}

#[tauri::command(async)]
pub fn get_who() -> Result<crate::who::WhoSnapshot, String> {
    crate::who::current()
}

#[tauri::command(async)]
pub fn get_w() -> Result<crate::w::WSnapshot, String> {
    crate::w::current()
}

#[tauri::command(async)]
pub fn get_system_profiler(
    query: crate::system_profiler::SystemProfilerQuery,
) -> Result<crate::system_profiler::SystemProfilerSnapshot, String> {
    crate::system_profiler::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_scutil(
    query: crate::scutil::ScutilQuery,
) -> Result<crate::scutil::ScutilResult, String> {
    crate::scutil::get_snapshot(query)
}

#[tauri::command(async)]
pub fn get_git(query: crate::git::GitQuery) -> Result<crate::git::GitResult, String> {
    crate::git::get_snapshot(query)
}

#[tauri::command(async)]
pub fn list_git_repos() -> Vec<String> {
    crate::git::list_repos()
}

#[tauri::command(async)]
pub fn set_remember(enabled: bool) {
    crate::privileged::set_remember(enabled);
}
