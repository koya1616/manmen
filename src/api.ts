import { invoke } from "@tauri-apps/api/core";
import type { CommandResult, DisablesleepState, TopSnapshot } from "./types";

// IPC コマンド名はここに集約する。Rust の commands.rs と一致させること。
export const api = {
  getDisablesleep: () => invoke<DisablesleepState>("get_disablesleep"),
  setDisablesleep: (enabled: boolean) =>
    invoke<CommandResult>("set_disablesleep", { enabled }),
  getManpage: (topic: string) =>
    invoke<CommandResult>("get_manpage", { topic }),
  getTop: (sortKey: string, count: number) =>
    invoke<TopSnapshot>("get_top", { sortKey, count }),
  setRemember: (enabled: boolean) =>
    invoke<void>("set_remember", { enabled }),
};
