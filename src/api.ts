import { invoke } from "@tauri-apps/api/core";
import type {
  CommandResult,
  DuSnapshot,
  InspectSnapshot,
  LsSnapshot,
  PmsetState,
  ManpageDocument,
  TopSnapshot,
  PsSnapshot,
  LsofSnapshot,
  DigSnapshot,
  SshSnapshot,
  SshKnownHost,
  TunnelInfo,
  TunnelStarted,
  VersionSnapshot,
  ContainerSnapshot,
  ImageSnapshot,
  NetworkSnapshot,
  VolumeSnapshot,
  SystemDfSnapshot,
  SystemInfoSnapshot,
  SystemProfilerSnapshot,
  ScutilSnapshot,
  GitSnapshot,
   WhoSnapshot,
   WSnapshot,
   PingSnapshot,
 } from "./types";

// IPC コマンド名はここに集約する。Rust の commands.rs と一致させること。
export const api = {
  getPmset: () => invoke<PmsetState>("get_pmset"),
  setPmset: (scope: string, setting: string, value: string) =>
    invoke<CommandResult>("set_pmset", { scope, setting, value }),
  getManpage: (topic: string) =>
    invoke<ManpageDocument>("get_manpage", { topic }),
  getDig: (query: {
    name: string;
    qtype: string;
    qclass: string;
    server: string;
    short: boolean;
    tcp: boolean;
    dnssec: boolean;
    trace: boolean;
    noRecurse: boolean;
    reverse: boolean;
    transport: string;
  }) => invoke<DigSnapshot>("get_dig", { query }),
  getSsh: (query: {
    host: string;
    user: string;
    port: number | null;
    mode: string;
    connectTimeout: number;
    strict: string;
    verbose: boolean;
    transport: string;
  }) => invoke<SshSnapshot>("get_ssh", { query }),
  startSshTunnel: (query: {
    mode: string;
    localHost: string;
    localPort: number;
    remoteHost: string;
    remotePort: number | null;
    host: string;
    user: string;
    port: number | null;
    keepalive: boolean;
    transport: string;
  }) => invoke<TunnelStarted>("start_ssh_tunnel", { query }),
  stopSshTunnel: (id: string) => invoke<void>("stop_ssh_tunnel", { id }),
  listSshTunnels: () => invoke<TunnelInfo[]>("list_ssh_tunnels"),
  listSshHosts: () => invoke<SshKnownHost[]>("list_ssh_hosts"),
  getPs: (query: {
    sort: string;
    columns: string[];
    user: string;
    pids: string;
  }) => invoke<PsSnapshot>("get_ps", { query }),
  getLsof: (query: {
    pids: string;
    user: string;
    comm: string;
    protocol: string;
    port: string;
    host: string;
    state: string;
  }) => invoke<LsofSnapshot>("get_lsof", { query }),
  getTop: (query: {
    sortKey: string;
    sortOrder: string;
    secondaryKey: string;
    count: number;
    countMode: string;
    noFrameworks: boolean;
    memoryMap: boolean;
    swap: boolean;
    user: string;
    pids: string;
    stats: string[];
    ncols: number | null;
  }) => invoke<TopSnapshot>("get_top", { query }),
  getWhoami: () => invoke<CommandResult>("get_whoami"),
  getWho: () => invoke<WhoSnapshot>("get_who"),
  getW: () => invoke<WSnapshot>("get_w"),
  getSystemProfiler: (dataType: string) =>
    invoke<SystemProfilerSnapshot>("get_system_profiler", { query: { dataType } }),
  getScutil: (sub: string) =>
    invoke<ScutilSnapshot>("get_scutil", { query: { sub } }),
  getGit: (sub: string, dir: string) =>
    invoke<GitSnapshot>("get_git", { query: { sub, dir } }),
  getPing: (query: {
    host: string;
    count: number;
    interval: number;
    timeout: number | null;
    waitMs: number | null;
    size: number;
    ttl: number | null;
    numeric: boolean;
    noFragment: boolean;
  }) => invoke<PingSnapshot>("get_ping", { query }),
  listGitRepos: () => invoke<string[]>("list_git_repos"),
  pruneDockerBuilder: (force: boolean, all: boolean) =>
    invoke<CommandResult>("prune_docker_builder", { force, all }),
  dockerBuilderDu: () => invoke<DuSnapshot>("docker_builder_du"),
  dockerBuilderLs: () => invoke<LsSnapshot>("docker_builder_ls"),
  dockerBuilderInspect: (name: string) =>
    invoke<InspectSnapshot>("docker_builder_inspect", { name }),
  dockerBuilderVersion: () =>
    invoke<VersionSnapshot>("docker_builder_version"),
  dockerContainers: () => invoke<ContainerSnapshot>("docker_containers"),
  dockerImages: () => invoke<ImageSnapshot>("docker_images"),
  dockerNetworks: () => invoke<NetworkSnapshot>("docker_networks"),
  dockerVolumes: () => invoke<VolumeSnapshot>("docker_volumes"),
  dockerSystemDf: () => invoke<SystemDfSnapshot>("docker_system_df"),
  dockerSystemInfo: () => invoke<SystemInfoSnapshot>("docker_system_info"),
  setRemember: (enabled: boolean) =>
    invoke<void>("set_remember", { enabled }),
};
