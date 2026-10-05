// Rust の lsof.rs ALLOWED_PROTOCOLS / ALLOWED_STATES と一致させること。
import { isValidUser as isValidUserBase, parsePidList as parsePidListBase } from "./validate";

export { isValidUserBase as isValidUser };

export const LSOF_PROTOCOLS = ["any", "TCP", "UDP"] as const;
export type LsofProtocol = (typeof LSOF_PROTOCOLS)[number];

export const LSOF_STATES = ["any", "LISTEN", "ESTABLISHED"] as const;
export type LsofState = (typeof LSOF_STATES)[number];

export const LSOF_DEFAULT_PROTOCOL: LsofProtocol = "any";
export const LSOF_DEFAULT_STATE: LsofState = "any";
export const LSOF_MAX_PIDS = 16;

const COMM_PATTERN = /^[A-Za-z0-9_][A-Za-z0-9._-]{0,31}$/;
const HOST_PATTERN = /^[A-Za-z0-9_.-]+$/;

export function parsePidList(raw: string): string[] | null {
  return parsePidListBase(raw, LSOF_MAX_PIDS);
}

export function isValidComm(comm: string): boolean {
  const trimmed = comm.trim();
  return trimmed === "" || COMM_PATTERN.test(trimmed);
}

export function isValidPort(raw: string): boolean {
  const port = raw.trim();
  if (port === "") return true;
  const parts = port.split("-");
  if (parts.length > 2) return false;
  return parts.every((part) => {
    if (!/^\d+$/.test(part)) return false;
    const n = Number(part);
    return n >= 1 && n <= 65535;
  });
}

export function isValidHost(raw: string): boolean {
  const host = raw.trim();
  if (host === "" || host.length > 253) return host === "";
  if (host.startsWith("-")) return false;
  if (/^\d+\.\d+\.\d+\.\d+$/.test(host)) {
    return host.split(".").every((part) => {
      const n = Number(part);
      return /^\d+$/.test(part) && n >= 0 && n <= 255;
    });
  }
  if (host.includes("@") || host.includes(" ") || host.includes("/") || host.includes(":")) {
    return false;
  }
  if (host.includes("..")) return false;
  return HOST_PATTERN.test(host);
}
