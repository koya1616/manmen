// Rust の ping.rs と制限を一致させること。

export const PING_DEFAULT_COUNT = 4;
export const PING_MAX_COUNT = 20;
export const PING_DEFAULT_INTERVAL = 1;
export const PING_MIN_INTERVAL = 0.2;
export const PING_MAX_INTERVAL = 10;
export const PING_DEFAULT_SIZE = 56;
export const PING_MAX_SIZE = 1472;
export const PING_MIN_WAIT_MS = 100;
export const PING_MAX_WAIT_MS = 10_000;
export const PING_DEFAULT_WAIT_MS = 1000;
export const PING_MIN_TIMEOUT = 1;
export const PING_MAX_TIMEOUT = 120;
export const PING_MIN_TTL = 1;
export const PING_MAX_TTL = 255;

const HOST_PATTERN = /^[A-Za-z0-9_.:-]+$/;

export function isValidPingHost(raw: string): boolean {
  const host = raw.trim();
  if (host === "" || host.length > 253) return false;
  if (host.startsWith("-") || host.startsWith("+") || host.startsWith("@")) return false;
  if (/[\s;|&`$()<>'"\\]/.test(host)) return false;
  const bare = host.split("%")[0];
  if (isIp(bare)) return true;
  const stripped = bare.endsWith(".") ? bare.slice(0, -1) : bare;
  if (stripped === "" || stripped.includes("..")) return false;
  return HOST_PATTERN.test(stripped);
}

export function isValidPingCount(raw: string): boolean {
  if (!/^\d+$/.test(raw.trim())) return false;
  const n = Number(raw.trim());
  return n >= 1 && n <= PING_MAX_COUNT;
}

export function isValidPingInterval(raw: string): boolean {
  const trimmed = raw.trim();
  if (trimmed === "") return false;
  const n = Number(trimmed);
  return Number.isFinite(n) && n >= PING_MIN_INTERVAL && n <= PING_MAX_INTERVAL;
}

export function isValidPingTimeout(raw: string): boolean {
  const trimmed = raw.trim();
  if (trimmed === "") return true;
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= PING_MIN_TIMEOUT && n <= PING_MAX_TIMEOUT;
}

export function isValidPingWaitMs(raw: string): boolean {
  const trimmed = raw.trim();
  if (trimmed === "") return true;
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= PING_MIN_WAIT_MS && n <= PING_MAX_WAIT_MS;
}

export function isValidPingSize(raw: string): boolean {
  const trimmed = raw.trim();
  if (trimmed === "") return false;
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= 0 && n <= PING_MAX_SIZE;
}

export function isValidPingTtl(raw: string): boolean {
  const trimmed = raw.trim();
  if (trimmed === "") return true;
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= PING_MIN_TTL && n <= PING_MAX_TTL;
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
    return /^[0-9a-fA-F:.]+$/.test(value) && value.length <= 45;
  }
  return false;
}
