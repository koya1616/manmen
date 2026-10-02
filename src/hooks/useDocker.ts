import { useState } from "react";
import { api } from "../api";
import {
  DEFAULT_DOCKER_ALL,
  DEFAULT_DOCKER_FORCE,
  buildDockerBuilderInspectPreview,
  buildDockerBuilderPrunePreview,
  isValidBuilderName,
} from "../commands/dockerOptions";
import type {
  CommandResult,
  DuSnapshot,
  InspectSnapshot,
  LsSnapshot,
  VersionSnapshot,
} from "../types";

export type DockerReadKind = "du" | "ls" | "inspect" | "version";

export type DockerResult =
  | { kind: "prune"; result: CommandResult }
  | { kind: "du"; result: DuSnapshot }
  | { kind: "ls"; result: LsSnapshot }
  | { kind: "inspect"; result: InspectSnapshot }
  | { kind: "version"; result: VersionSnapshot };

export function useDocker() {
  const [force, setForce] = useState(DEFAULT_DOCKER_FORCE);
  const [all, setAll] = useState(DEFAULT_DOCKER_ALL);
  const [name, setName] = useState("");
  const [executing, setExecuting] = useState(false);
  const [dockerResult, setDockerResult] = useState<DockerResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const preview = buildDockerBuilderPrunePreview(force, all);
  const inspectPreview = buildDockerBuilderInspectPreview(name);
  const nameOk = isValidBuilderName(name);

  async function run<T>(task: () => Promise<T>, wrap: (result: T) => DockerResult) {
    setExecuting(true);
    setDockerResult(null);
    setError(null);
    try {
      const res = await task();
      setDockerResult(wrap(res));
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  async function execute() {
    await run(() => api.pruneDockerBuilder(force, all), (result) => ({
      kind: "prune",
      result,
    }));
  }

  async function executeRead(kind: DockerReadKind) {
    if (kind === "inspect" && !nameOk) return;
    switch (kind) {
      case "du":
        await run(() => api.dockerBuilderDu(), (result) => ({ kind: "du", result }));
        break;
      case "ls":
        await run(() => api.dockerBuilderLs(), (result) => ({ kind: "ls", result }));
        break;
      case "inspect":
        await run(() => api.dockerBuilderInspect(name.trim()), (result) => ({
          kind: "inspect",
          result,
        }));
        break;
      case "version":
        await run(() => api.dockerBuilderVersion(), (result) => ({
          kind: "version",
          result,
        }));
        break;
    }
  }

  return {
    force,
    setForce,
    all,
    setAll,
    name,
    setName,
    nameOk,
    executing,
    dockerResult,
    error,
    preview,
    inspectPreview,
    execute,
    executeRead,
  };
}
