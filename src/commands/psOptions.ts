// Rust の ps.rs ALLOWED_COLUMNS と一致させること。
import { isValidUser as isValidUserBase, parsePidList as parsePidListBase } from "./validate";

export const PS_COLUMNS = [
  "pid",
  "ppid",
  "pgid",
  "user",
  "%cpu",
  "%mem",
  "rss",
  "vsz",
  "time",
  "etime",
  "state",
  "tty",
  "nice",
  "comm",
  "command",
  "start",
] as const;

export type PsColumn = (typeof PS_COLUMNS)[number];
export type PsSort = "none" | "cpu" | "mem";

export const PS_DEFAULT_COLUMNS: PsColumn[] = [
  "pid",
  "user",
  "%cpu",
  "%mem",
  "rss",
  "time",
  "state",
  "command",
];
export const PS_DEFAULT_SORT: PsSort = "none";
export const PS_MAX_PIDS = 16;
export const PS_MAX_COLUMNS = 16;

export { isValidUserBase as isValidUser };

export function parsePidList(raw: string): string[] | null {
  return parsePidListBase(raw, PS_MAX_PIDS);
}
