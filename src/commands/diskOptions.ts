// Rust の df.rs / du.rs と制限を一致させること。

export const DF_FS_TYPES = ["apfs", "hfs", "msdos", "exfat", "smbfs", "nfs"] as const;

export const DU_DEFAULT_DEPTH = 1;
export const DU_MAX_DEPTH = 3;
export const DU_DEFAULT_TIMEOUT_SECS = 30;
export const DU_MIN_TIMEOUT_SECS = 5;
export const DU_MAX_TIMEOUT_SECS = 120;

export function isValidDuDepth(raw: string): boolean {
  return isIntInRange(raw, 0, DU_MAX_DEPTH);
}

export function isValidDuTimeout(raw: string): boolean {
  return isIntInRange(raw, DU_MIN_TIMEOUT_SECS, DU_MAX_TIMEOUT_SECS);
}

function isIntInRange(raw: string, min: number, max: number): boolean {
  const trimmed = raw.trim();
  if (!/^\d+$/.test(trimmed)) return false;
  const n = Number(trimmed);
  return n >= min && n <= max;
}
