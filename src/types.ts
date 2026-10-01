// Rust の types.rs / pmset.rs / top.rs とフィールドを一致させること。

export interface CommandResult {
  success: boolean;
  exit_code: number;
  stdout: string;
  stderr: string;
  command: string;
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
}

export interface TopProcess {
  pid: number;
  command: string;
  cpu: number;
  time: string;
  threads: number;
  ports: number;
  mem: string;
  state: string;
  user: string;
}

export interface TopSnapshot {
  success: boolean;
  exit_code: number;
  command: string;
  summary: TopSummary;
  processes: TopProcess[];
  stderr: string;
}

export interface DisablesleepState {
  enabled: boolean | null;
}
