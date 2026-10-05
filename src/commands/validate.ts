// ps / top / lsof / ssh で重複していたユーザ名・PIDリスト検証の共通実装。
// 各 *Options.ts の MAX 定数と Rust 側の制限に合わせるため、上限は引数で受ける。

export const USER_PATTERN = /^[A-Za-z0-9_][A-Za-z0-9._-]{0,31}$/;

export function isValidUser(user: string): boolean {
  const trimmed = user.trim();
  return trimmed === "" || USER_PATTERN.test(trimmed);
}

export function parsePidList(raw: string, max: number): string[] | null {
  const tokens = raw
    .split(/[\s,]+/)
    .map((token) => token.trim())
    .filter(Boolean);
  const unique: string[] = [];
  for (const token of tokens) {
    if (!/^\d+$/.test(token)) return null;
    if (!unique.includes(token)) unique.push(token);
  }
  if (unique.length > max) return null;
  return unique;
}
