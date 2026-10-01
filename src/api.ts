import { invoke } from "@tauri-apps/api/core";
import type {
  CommandResult,
  PmsetState,
  ManpageDocument,
  TopSnapshot,
} from "./types";

// IPC コマンド名はここに集約する。Rust の commands.rs と一致させること。
export const api = {
  getPmset: () => invoke<PmsetState>("get_pmset"),
  setPmset: (scope: string, setting: string, value: string) =>
    invoke<CommandResult>("set_pmset", { scope, setting, value }),
  getManpage: (topic: string) =>
    invoke<ManpageDocument>("get_manpage", { topic }),
  getTop: (sortKey: string, count: number) =>
    invoke<TopSnapshot>("get_top", { sortKey, count }),
  setRemember: (enabled: boolean) =>
    invoke<void>("set_remember", { enabled }),
};
