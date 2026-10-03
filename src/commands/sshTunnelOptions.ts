// Rust の ssh.rs トンネル管理 (TUNNEL_* / ALLOWED_BINDS) と一致させること。

import { isValidSshHost, isValidSshPort, isValidSshUser } from "./sshOptions";

export const TUNNEL_MODES = ["local", "remote", "dynamic"] as const;

export type TunnelMode = (typeof TUNNEL_MODES)[number];

export const TUNNEL_BINDS = ["", "127.0.0.1", "localhost"];

export function isValidTunnelBind(raw: string): boolean {
  return TUNNEL_BINDS.includes(raw.trim());
}

export function isValidRequiredPort(raw: string): boolean {
  if (!/^\d+$/.test(raw.trim())) return false;
  const n = Number(raw);
  return n >= 1 && n <= 65535;
}

export function isValidTunnelRemoteHost(raw: string, mode: TunnelMode): boolean {
  if (mode === "dynamic") return true;
  return isValidSshHost(raw);
}

export function isValidTunnelRemotePort(raw: string, mode: TunnelMode): boolean {
  if (mode === "dynamic") return true;
  return isValidRequiredPort(raw);
}

export { isValidSshHost as isValidTunnelSshHost };
export { isValidSshPort as isValidTunnelSshPort };
export { isValidSshUser as isValidTunnelSshUser };
