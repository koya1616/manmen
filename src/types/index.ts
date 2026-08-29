export interface CommandSummary {
  id: string;
  name: string;
  description: string;
  category: string;
  requires_admin: boolean;
  dangerous: boolean;
  tags: string[];
}

export interface CommandDetail {
  id: string;
  command: string;
  name: string;
  category: string;
  description: string;
  requires_admin: boolean;
  dangerous: boolean;
  arguments: ArgumentDef[];
  timeout_ms: number | null;
  tags: string[];
}

export interface ArgumentDef {
  id: string;
  name: string;
  description: string;
  type: ArgumentType;
  required: boolean;
  default: unknown;
  min?: number;
  max?: number;
  unit?: string;
  options?: string[];
  cli_flag?: string;
  cli_key?: string;
}

export type ArgumentType =
  | "string"
  | "integer"
  | "number"
  | "boolean"
  | "enum"
  | "path"
  | "file"
  | "directory"
  | "duration"
  | "multiple";

export interface CommandCategory {
  id: string;
  name: string;
  description: string | null;
}

export interface CommandValidationResult {
  valid: boolean;
  errors: ValidationError[];
}

export interface ValidationError {
  field: string;
  code: string;
  message: string;
}

export interface BuildCommandResponse {
  program: string;
  args: string[];
  full_command: string;
}

export interface CommandExecutionRequest {
  command_id: string;
  arguments: Record<string, unknown>;
}

export interface CommandExecutionResponse {
  execution_id: string;
  success: boolean;
  exit_code: number;
  stdout: string;
  stderr: string;
  duration_ms: number;
}

export interface HistoryEntry {
  id: string;
  command_id: string;
  command_name: string;
  generated_command: string;
  success: boolean;
  exit_code: number | null;
  duration_ms: number | null;
  started_at: string;
}

export interface HistoryDetail {
  id: string;
  command_id: string;
  command_name: string;
  generated_command: string;
  success: boolean;
  exit_code: number | null;
  stdout: string;
  stderr: string;
  duration_ms: number | null;
  started_at: string;
}
