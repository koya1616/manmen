// Rust の kill.rs と制限を一致させること。
import { parsePidList } from "./validate";

export const KILL_MAX_PIDS = 10;
export const KILL_SIGNALS = ["TERM", "INT", "HUP", "QUIT", "KILL", "STOP", "CONT"] as const;
export type KillSignal = (typeof KILL_SIGNALS)[number];

// PID 0 (プロセスグループ全体) と 1 (launchd) は送信対象にしない。
const FORBIDDEN_PIDS = ["0", "1"];

export function parseKillPids(raw: string): string[] | null {
  const pids = parsePidList(raw, KILL_MAX_PIDS);
  if (pids === null || pids.length === 0) return null;
  if (pids.some((pid) => FORBIDDEN_PIDS.includes(String(Number(pid))))) return null;
  return pids;
}
