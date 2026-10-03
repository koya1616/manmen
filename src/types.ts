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
