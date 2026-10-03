// Rust の dig.rs ALLOWED_TYPES / ALLOWED_CLASSES と一致させること。

export const DIG_TYPES = [
  "A",
  "AAAA",
  "CNAME",
  "MX",
  "NS",
  "SOA",
  "TXT",
  "SRV",
  "CAA",
  "PTR",
] as const;

export type DigType = (typeof DIG_TYPES)[number];

export const DIG_CLASSES = ["IN", "CH", "HS"] as const;

export type DigClass = (typeof DIG_CLASSES)[number];
export type DigTransport = "auto" | "4" | "6";

export const DIG_DEFAULT_TYPE: DigType = "A";
export const DIG_DEFAULT_CLASS: DigClass = "IN";

const NAME_PATTERN = /^[A-Za-z0-9_.-]+$/;

export function isValidDigName(raw: string, reverse: boolean): boolean {
  const name = raw.trim();
  if (name === "") return false;
  if (name.length > 253) return false;
  if (name.startsWith("-") || name.startsWith("+") || name.startsWith("@")) return false;
  if (reverse) return isIp(name);
  const stripped = name.endsWith(".") ? name.slice(0, -1) : name;
  if (stripped === "" || stripped.includes("..")) return false;
  return NAME_PATTERN.test(stripped);
}

export function isValidDigServer(raw: string): boolean {
  const server = raw.trim();
  if (server === "") return true;
  const bare = server.startsWith("@") ? server.slice(1) : server;
  if (bare === "" || bare.length > 253) return false;
  if (bare.startsWith("-") || bare.startsWith("+")) return false;
  if (isIp(bare)) return true;
  const stripped = bare.endsWith(".") ? bare.slice(0, -1) : bare;
  if (stripped === "" || stripped.includes("..")) return false;
  return NAME_PATTERN.test(stripped);
}

function isIp(value: string): boolean {
  // IPv4 の簡易判定 + IPv6 はコロン含みの16進とみなす
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
