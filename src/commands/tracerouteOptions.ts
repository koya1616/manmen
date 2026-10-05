// Rust の traceroute.rs と制限を一致させること。

export const TRACEROUTE_DEFAULT_MAX_TTL = 20;
export const TRACEROUTE_MAX_MAX_TTL = 30;
export const TRACEROUTE_DEFAULT_QUERIES = 3;
export const TRACEROUTE_MAX_QUERIES = 3;
export const TRACEROUTE_DEFAULT_WAIT = 2;
export const TRACEROUTE_MAX_WAIT = 5;
export const TRACEROUTE_MAX_WORST_SECS = 150;

export function isValidTracerouteMaxTtl(raw: string): boolean {
  return isIntInRange(raw, 1, TRACEROUTE_MAX_MAX_TTL);
}

// 空欄は「1 から」として許可する。
export function isValidTracerouteFirstTtl(raw: string, maxTtl: number): boolean {
  if (raw.trim() === "") return true;
  return isIntInRange(raw, 1, maxTtl);
}

export function isValidTracerouteQueries(raw: string): boolean {
  return isIntInRange(raw, 1, TRACEROUTE_MAX_QUERIES);
}

export function isValidTracerouteWait(raw: string): boolean {
  return isIntInRange(raw, 1, TRACEROUTE_MAX_WAIT);
}

// 応答が全く無い場合の最悪所要時間 (秒)。
export function tracerouteWorstSecs(input: {
  maxTtl: number;
  firstTtl: number | null;
  queries: number;
  wait: number;
}): number {
  const hops = input.maxTtl - (input.firstTtl ?? 1) + 1;
  return Math.max(0, hops) * input.queries * input.wait;
}

function isIntInRange(raw: string, min: number, max: number): boolean {
  const trimmed = raw.trim();
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= min && n <= max;
}
