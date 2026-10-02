// Rust の types.rs / pmset.rs / manpage.rs / top.rs / docker.rs とフィールドを一致させること.

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
