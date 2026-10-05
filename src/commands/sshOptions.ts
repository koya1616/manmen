// Rust の ssh.rs ALLOWED_STRICT / FEATURED_KEYS / MIN_TIMEOUT 等と一致させること。
import { isValidUser } from "./validate";

export const SSH_MODES = ["config", "test"] as const;

export type SshMode = (typeof SSH_MODES)[number];

export const SSH_STRICTS = ["ask", "accept-new", "yes"] as const;

export type SshStrict = (typeof SSH_STRICTS)[number];
export type SshTransport = "auto" | "4" | "6";

export const SSH_DEFAULT_MODE: SshMode = "config";
export const SSH_DEFAULT_STRICT: SshStrict = "accept-new";
export const SSH_DEFAULT_TIMEOUT = 10;
export const SSH_MIN_TIMEOUT = 5;
export const SSH_MAX_TIMEOUT = 30;
export const SSH_MIN_PORT = 1;
export const SSH_MAX_PORT = 65535;

export const SSH_FEATURED_KEYS = [
  "hostname",
  "user",
  "port",
  "identityfile",
  "stricthostkeychecking",
  "batchmode",
  "connecttimeout",
  "proxyjump",
  "serveraliveinterval",
  "serveralivecountmax",
  "forwardagent",
  "identitiesonly",
];

const HOST_PATTERN = /^[A-Za-z0-9_.:-]+$/;

export function isValidSshHost(raw: string): boolean {
  const host = raw.trim();
  if (host === "" || host.length > 253) return false;
  if (host.startsWith("-")) return false;
  if (isIp(host)) return true;
  if (host.includes("@") || host.includes(" ") || host.includes("/")) return false;
  if (host.includes("..")) return false;
  return HOST_PATTERN.test(host);
}

export function isValidSshUser(raw: string): boolean {
  return isValidUser(raw);
}

export function isValidSshPort(raw: string): boolean {
  if (raw.trim() === "") return true;
  if (!/^\d+$/.test(raw.trim())) return false;
  const n = Number(raw);
  return n >= SSH_MIN_PORT && n <= SSH_MAX_PORT;
}

export function isValidSshTimeout(raw: string): boolean {
  if (!/^\d+$/.test(raw.trim())) return false;
  const n = Number(raw);
  return n >= SSH_MIN_TIMEOUT && n <= SSH_MAX_TIMEOUT;
}

function isIp(value: string): boolean {
  if (/^\d+\.\d+\.\d+\.\d+$/.test(value)) {
    return value.split(".").every((part) => {
      if (!/^\d+$/.test(part)) return false;
      const n = Number(part);
      return n >= 0 && n <= 255;
    });
  }
  if (value.includes(":")) {
    return /^[0-9a-fA-F:.%]+$/.test(value) && value.length <= 45;
  }
  return false;
}
