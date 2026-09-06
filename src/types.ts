// Rust の types.rs / pmset.rs とフィールドを一致させること。

export interface CommandResult {
  success: boolean;
  exit_code: number;
  stdout: string;
  stderr: string;
  command: string;
}

export interface DisablesleepState {
  enabled: boolean | null;
}
