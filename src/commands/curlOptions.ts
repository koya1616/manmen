// Rust の curl.rs と制限を一致させること。

export const CURL_DEFAULT_MAX_TIME = 30;
export const CURL_MIN_MAX_TIME = 1;
export const CURL_MAX_MAX_TIME = 120;
export const CURL_MIN_CONNECT_TIMEOUT = 1;
export const CURL_MAX_CONNECT_TIMEOUT = 60;
export const CURL_DEFAULT_MAX_REDIRS = 10;
export const CURL_MAX_MAX_REDIRS = 20;
export const CURL_MAX_URL_LEN = 2048;
export const CURL_MAX_HEADERS = 5;
export const CURL_MAX_HEADER_LEN = 1024;

export type CurlHttpVersion = "" | "1.1" | "2";
export type CurlIpVersion = "" | "4" | "6";

const HEADER_NAME_PATTERN = /^[A-Za-z0-9!#$%&'*+\-.^_`|~]+$/;

export function isValidCurlUrl(raw: string): boolean {
  const url = raw.trim();
  if (url === "" || url.length > CURL_MAX_URL_LEN) return false;
  if (/\s/.test(url) || hasControlChar(url)) return false;
  const match = /^https?:\/\/(.*)$/i.exec(url);
  if (!match) return false;
  const authority = match[1].split(/[/?#]/)[0];
  const host = authority.split("@").pop() ?? "";
  return host !== "" && !host.startsWith(":");
}

// 空欄は「未入力」として許可する (送信時に除外する)。
export function isValidCurlHeader(raw: string): boolean {
  const header = raw.trim();
  if (header === "") return true;
  if (header.length > CURL_MAX_HEADER_LEN) return false;
  const colon = header.indexOf(":");
  if (colon <= 0) return false;
  const name = header.slice(0, colon);
  const value = header.slice(colon + 1).replace(/\t/g, " ");
  return HEADER_NAME_PATTERN.test(name) && !hasControlChar(value);
}

export function isValidCurlMaxTime(raw: string): boolean {
  return isIntInRange(raw, CURL_MIN_MAX_TIME, CURL_MAX_MAX_TIME);
}

export function isValidCurlConnectTimeout(raw: string): boolean {
  if (raw.trim() === "") return true;
  return isIntInRange(raw, CURL_MIN_CONNECT_TIMEOUT, CURL_MAX_CONNECT_TIMEOUT);
}

export function isValidCurlMaxRedirs(raw: string): boolean {
  return isIntInRange(raw, 0, CURL_MAX_MAX_REDIRS);
}

function hasControlChar(value: string): boolean {
  for (const ch of value) {
    const code = ch.charCodeAt(0);
    if (code < 0x20 || code === 0x7f) return true;
  }
  return false;
}

function isIntInRange(raw: string, min: number, max: number): boolean {
  const trimmed = raw.trim();
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= min && n <= max;
}
