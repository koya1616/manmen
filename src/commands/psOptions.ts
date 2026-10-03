// Rust の ps.rs ALLOWED_COLUMNS と一致させること。

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
  if (unique.length > PS_MAX_PIDS) return null;
  return unique;
}

export function isValidUser(user: string): boolean {
  const trimmed = user.trim();
  return trimmed === "" || USER_PATTERN.test(trimmed);
}
