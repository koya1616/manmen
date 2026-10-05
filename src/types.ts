// Rust の types.rs / pmset.rs / manpage.rs / top.rs / ps.rs / docker.rs とフィールドを一致させること.

export interface CommandResult {
  success: boolean;
  exit_code: number;
  stdout: string;
  stderr: string;
  command: string;
}

export interface ManInline {
  kind: string;
  text: string;
  children: ManInline[];
}

export interface ManBlock {
  kind: string;
  inlines: ManInline[];
}

export interface ManSection {
  title: string;
  blocks: ManBlock[];
}

export interface ManpageDocument {
  success: boolean;
  exit_code: number;
  command: string;
  title: string;
  sections: ManSection[];
  stderr: string;
}

export interface TopSummary {
  timestamp: string;
  processes_total: number | null;
  processes_running: number | null;
  processes_sleeping: number | null;
  threads: number | null;
  load_avg_1: number | null;
  load_avg_5: number | null;
  load_avg_15: number | null;
  cpu_user: number | null;
  cpu_sys: number | null;
  cpu_idle: number | null;
  physmem: string;
  extra: string[];
}

export interface TopSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  summary: TopSummary;
  columns: string[];
  rows: string[][];
  stderr: string;
}

export interface PsSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  columns: string[];
  rows: string[][];
  count: number;
  stderr: string;
}

export interface LsofSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  columns: string[];
  rows: string[][];
  count: number;
  stderr: string;
}

export interface PmsetValue {
  name: string;
  value: string;
}

export interface PmsetState {
  battery: PmsetValue[];
  ac: PmsetValue[];
  ups: PmsetValue[];
  sleep_disabled: string | null;
}

export interface DuEntry {
  id: string;
  reclaimable: boolean;
  shared: boolean;
  size: string;
  last_accessed: string;
}

export interface DuSummary {
  shared: string;
  private: string;
  reclaimable: string;
  total: string;
}

export interface DuSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  entries: DuEntry[];
  summary: DuSummary;
  stderr: string;
}

export interface BuilderNode {
  name: string;
  endpoint: string;
  status: string;
  buildkit: string;
  platforms: string;
}

export interface BuilderInstance {
  name: string;
  driver: string;
  is_current: boolean;
  nodes: BuilderNode[];
}

export interface LsSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  builders: BuilderInstance[];
  stderr: string;
}

export interface BuilderVersion {
  package: string;
  version: string;
  commit: string;
}

export interface VersionSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  version: BuilderVersion;
  raw: string;
  stderr: string;
}

export interface GcPolicy {
  name: string;
  all: string;
  filters: string;
  keep_duration: string;
  max_used_space: string;
  reserved_space: string;
  min_free_space: string;
  extra: string[];
}

export interface InspectNode {
  name: string;
  endpoint: string;
  status: string;
  buildkit: string;
  platforms: string;
  labels: string[];
  devices: string[];
  gc_policies: GcPolicy[];
}

export interface InspectSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  name: string;
  driver: string;
  last_activity: string;
  nodes: InspectNode[];
  stderr: string;
}

// Rust の docker_read.rs とフィールドを一致させること.

export interface DockerContainerEntry {
  id: string;
  names: string;
  image: string;
  command: string;
  created: string;
  status: string;
  state: string;
  ports: string;
}

export interface ContainerSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  containers: DockerContainerEntry[];
  stderr: string;
}

export interface DockerImageEntry {
  repository: string;
  tag: string;
  id: string;
  created_since: string;
  size: string;
  containers: string;
}

export interface ImageSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  images: DockerImageEntry[];
  stderr: string;
}

export interface DockerNetworkEntry {
  id: string;
  name: string;
  driver: string;
  scope: string;
}

export interface NetworkSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  networks: DockerNetworkEntry[];
  stderr: string;
}

export interface DockerVolumeEntry {
  driver: string;
  name: string;
  scope: string;
}

export interface VolumeSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  volumes: DockerVolumeEntry[];
  stderr: string;
}

export interface SystemDfEntry {
  dtype: string;
  total: string;
  active: string;
  size: string;
  reclaimable: string;
}

export interface SystemDfSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  entries: SystemDfEntry[];
  stderr: string;
}

export interface SystemInfoData {
  containers: number;
  containers_running: number;
  containers_stopped: number;
  images: number;
  driver: string;
  server_version: string;
  operating_system: string;
  architecture: string;
  ncpu: number;
  mem_total: number;
  kernel_version: string;
}

export interface SystemInfoSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  info: SystemInfoData;
  stderr: string;
}

export interface DigRecord {
  name: string;
  ttl: number | null;
  class: string;
  dtype: string;
  value: string;
}

export interface DigSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  answers: DigRecord[];
  query_time: string;
  server: string;
  when: string;
  msg_size: string;
  raw: string;
  stderr: string;
}

export interface SshConfigEntry {
  key: string;
  value: string;
}

export interface SshSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  mode: string;
  entries: SshConfigEntry[];
  stdout: string;
  raw: string;
  stderr: string;
}

export interface TunnelStarted {
  id: string;
  command: string;
  summary: string;
}

export interface TunnelInfo {
  id: string;
  command: string;
  summary: string;
  mode: string;
}

export interface SshKnownHost {
  alias: string;
  hostname: string | null;
  user: string | null;
  port: number | null;
}

// Rust の who.rs とフィールドを一致させること.
export interface WhoEntry {
  user: string;
  tty: string;
  login: string;
}

export interface WhoSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  users: WhoEntry[];
  count: number;
  stderr: string;
}

// Rust の w.rs とフィールドを一致させること.
export interface WSummary {
  headline: string;
  load_avg_1: number | null;
  load_avg_5: number | null;
  load_avg_15: number | null;
}

export interface WEntry {
  user: string;
  tty: string;
  from: string;
  login: string;
  idle: string;
  what: string;
}

export interface WSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  summary: WSummary;
  users: WEntry[];
  count: number;
  stderr: string;
}

// Rust の system_profiler.rs とフィールドを一致させること.
// 型ごとにスキーマが違うため JSON を汎用ツリーで表示する.
export type ProfilerValue =
  | string
  | number
  | boolean
  | null
  | ProfilerValue[]
  | { [key: string]: ProfilerValue };

export interface SystemProfilerSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  data_type: string;
  value: ProfilerValue;
  stderr: string;
}

// Rust の scutil.rs とフィールドを一致させること.
export interface ScutilEntry {
  key: string;
  value: string;
}

export interface DnsResolver {
  name: string;
  entries: ScutilEntry[];
}

export interface DnsSection {
  title: string;
  resolvers: DnsResolver[];
}

export interface DnsSnapshot {
  kind: "dns";
  success: boolean;
  exit_code: number;
  command: string;
  sections: DnsSection[];
  stderr: string;
}

export interface NwiRow {
  iface: string;
  key: string;
  value: string;
}

export interface NwiSection {
  title: string;
  rows: NwiRow[];
}

export interface NwiSnapshot {
  kind: "nwi";
  success: boolean;
  exit_code: number;
  command: string;
  sections: NwiSection[];
  footer: string;
  stderr: string;
}

export interface NameEntry {
  key: string;
  value: string;
}

export interface NamesSnapshot {
  kind: "names";
  success: boolean;
  exit_code: number;
  command: string;
  names: NameEntry[];
  stderr: string;
}

export type ScutilSnapshot = DnsSnapshot | NwiSnapshot | NamesSnapshot;

// Rust の git.rs とフィールドを一致させること.
export interface GitFile {
  xy: string;
  path: string;
}

export interface GitStatusSnapshot {
  kind: "status";
  success: boolean;
  exit_code: number;
  command: string;
  dir: string;
  branch: string;
  tracking: string;
  files: GitFile[];
  count: number;
  stderr: string;
}

export interface GitCommit {
  hash: string;
  author: string;
  date: string;
  subject: string;
}

export interface GitLogSnapshot {
  kind: "log";
  success: boolean;
  exit_code: number;
  command: string;
  dir: string;
  commits: GitCommit[];
  count: number;
  stderr: string;
}

export interface GitBranch {
  name: string;
  current: boolean;
  remote: boolean;
}

export interface GitBranchesSnapshot {
  kind: "branches";
  success: boolean;
  exit_code: number;
  command: string;
  dir: string;
  branches: GitBranch[];
  count: number;
  stderr: string;
}

export interface GitRemote {
  name: string;
  url: string;
  kind: string;
}

export interface GitRemotesSnapshot {
  kind: "remotes";
  success: boolean;
  exit_code: number;
  command: string;
  dir: string;
  remotes: GitRemote[];
  count: number;
  stderr: string;
}

export type GitSnapshot =
  | GitStatusSnapshot
  | GitLogSnapshot
  | GitBranchesSnapshot
  | GitRemotesSnapshot;

// Rust の ping.rs とフィールドを一致させること.
export interface PingReply {
  seq: number;
  bytes: number | null;
  from: string;
  ttl: number | null;
  time_ms: number | null;
  timeout: boolean;
}

export interface PingStats {
  transmitted: number;
  received: number;
  loss_percent: number;
  min_ms: number | null;
  avg_ms: number | null;
  max_ms: number | null;
  stddev_ms: number | null;
}

export interface PingSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  target: string;
  resolved_ip: string;
  replies: PingReply[];
  timeout_count: number;
  stats: PingStats;
  raw: string;
  stderr: string;
}

// Rust の curl.rs とフィールドを一致させること.
export interface CurlHeader {
  name: string;
  value: string;
}

export interface CurlResponse {
  status_line: string;
  http_version: string;
  status_code: number | null;
  headers: CurlHeader[];
}

export interface CurlInfo {
  http_code: number;
  http_version: string;
  method: string | null;
  scheme: string | null;
  remote_ip: string | null;
  remote_port: number | null;
  url_effective: string | null;
  num_redirects: number;
  content_type: string | null;
  size_download: number;
  size_header: number;
  speed_download: number;
  ssl_verify_result: number;
  time_namelookup: number;
  time_connect: number;
  time_appconnect: number;
  time_pretransfer: number;
  time_redirect: number;
  time_starttransfer: number;
  time_total: number;
  errormsg: string | null;
}

export interface CurlSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  responses: CurlResponse[];
  body: string;
  body_truncated: boolean;
  body_binary: boolean;
  info: CurlInfo | null;
  stderr: string;
}

// Rust の kill.rs とフィールドを一致させること.
export interface KillTarget {
  pid: number;
  found: boolean;
  user: string;
  command: string;
}

export interface KillPidResult {
  pid: number;
  ok: boolean;
  error: string;
}

export interface KillSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  signal: string;
  results: KillPidResult[];
  stderr: string;
}
