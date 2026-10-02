// Rust の docker.rs と入力検証を一致させること。
// `docker builder prune` の `-f` / `--all` のみ許可する。
// 参照系 (du / ls / inspect / version) は表示専用。

export const DEFAULT_DOCKER_FORCE = true;
export const DEFAULT_DOCKER_ALL = false;

const BUILDER_NAME_PATTERN = /^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/;

export function isValidBuilderName(raw: string): boolean {
  const name = raw.trim();
  return name === "" || BUILDER_NAME_PATTERN.test(name);
}
