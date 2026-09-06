import { invoke } from "@tauri-apps/api/core";
import type { CommandResult, DisablesleepState } from "./types";

// IPC コマンド名はここに集約する。Rust の commands.rs と一致させること。
export const api = {
  getDisablesleep: () => invoke<DisablesleepState>("get_disablesleep"),
  setDisablesleep: (enabled: boolean) =>
    invoke<CommandResult>("set_disablesleep", { enabled }),
  setRemember: (enabled: boolean) =>
    invoke<void>("set_remember", { enabled }),
};
