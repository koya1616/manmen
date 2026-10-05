// KiB 単位の値を読みやすい単位に変換する (df / du で共通)。
export function formatKb(kb: number): string {
  const units = ["KB", "MB", "GB", "TB"];
  let value = kb;
  let unit = 0;
  while (value >= 1024 && unit < units.length - 1) {
    value /= 1024;
    unit += 1;
  }
  return `${value.toFixed(unit === 0 || value >= 100 ? 0 : 1)} ${units[unit]}`;
}
