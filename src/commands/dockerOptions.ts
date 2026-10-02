// Rust の docker.rs とプレビューを一致させること。
// `docker builder prune` の `-f` / `--all` のみ許可する。
// 参照系 (du / ls / inspect / version) は表示専用。

export const DEFAULT_DOCKER_FORCE = true;
export const DEFAULT_DOCKER_ALL = false;

export function buildDockerBuilderPrunePreview(force: boolean, all: boolean): string {
  const parts = ["docker", "builder", "prune"];
  if (force) parts.push("-f");
  if (all) parts.push("--all");
  return parts.join(" ");
}

const BUILDER_NAME_PATTERN = /^[A-Za-z0-9][A-Za-z0-9._-]{0,63}$/;

export function isValidBuilderName(raw: string): boolean {
  const name = raw.trim();
  return name === "" || BUILDER_NAME_PATTERN.test(name);
}

export function buildDockerBuilderInspectPreview(raw: string): string {
  const name = raw.trim();
  return name === "" ? "docker builder inspect" : `docker builder inspect ${name}`;
}
