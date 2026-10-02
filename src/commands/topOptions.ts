// Rust の top.rs ALLOWED_KEYS と一致させること。

export const TOP_KEYS = [
  "pid",
  "command",
  "cpu",
  "cpu_me",
  "cpu_others",
  "csw",
  "time",
  "threads",
  "ports",
  "mregion",
  "mem",
  "rprvt",
  "purg",
  "vsize",
  "vprvt",
  "kprvt",
  "kshrd",
  "pgrp",
  "ppid",
  "state",
  "uid",
  "wq",
  "faults",
  "cow",
  "user",
  "msgsent",
  "msgrecv",
  "sysbsd",
  "sysmach",
  "pageins",
  "boosts",
  "instrs",
  "cycles",
  "jetpri",
] as const;

export type TopKey = (typeof TOP_KEYS)[number];
export type TopSortOrder = "" | "+" | "-";
export type TopCountMode = "n" | "a" | "d" | "e";

export const TOP_DEFAULT_SORT: TopKey = "cpu";
export const TOP_DEFAULT_COUNT = 20;
export const TOP_MAX_COUNT = 100;
export const TOP_MAX_PIDS = 16;
export const TOP_MAX_STATS = 16;
export const TOP_MIN_NCOLS = 40;
export const TOP_MAX_NCOLS = 400;

const USER_PATTERN = /^[A-Za-z0-9_][A-Za-z0-9._-]{0,31}$/;

export function parsePidList(raw: string): string[] | null {
  const tokens = raw
    .split(/[\s,]+/)
    .map((token) => token.trim())
    .filter(Boolean);
  const unique: string[] = [];
  for (const token of tokens) {
    if (!/^\d+$/.test(token)) return null;
    if (!unique.includes(token)) unique.push(token);
  }
  if (unique.length > TOP_MAX_PIDS) return null;
  return unique;
}

export function isValidUser(user: string): boolean {
  const trimmed = user.trim();
  return trimmed === "" || USER_PATTERN.test(trimmed);
}

export function isValidNcols(raw: string): boolean {
  if (raw.trim() === "") return true;
  if (!/^\d+$/.test(raw.trim())) return false;
  const n = Number(raw);
  return n >= TOP_MIN_NCOLS && n <= TOP_MAX_NCOLS;
}
