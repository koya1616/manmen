import { useState } from "react";
import { api } from "../api";
import {
  DEFAULT_DOCKER_ALL,
  DEFAULT_DOCKER_FORCE,
  isValidBuilderName,
} from "../commands/dockerOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type {
  CommandResult,
  DuSnapshot,
  InspectSnapshot,
  LsSnapshot,
  VersionSnapshot,
} from "../types";
import { useRunner } from "./useRunner";

export type DockerSub = "du" | "ls" | "inspect" | "version" | "prune";

export const DOCKER_SUBS: DockerSub[] = ["du", "ls", "inspect", "version", "prune"];

export type DockerResult =
  | { kind: "prune"; result: CommandResult }
  | { kind: "du"; result: DuSnapshot }
  | { kind: "ls"; result: LsSnapshot }
  | { kind: "inspect"; result: InspectSnapshot }
  | { kind: "version"; result: VersionSnapshot };

// Rust の docker.rs が組み立てるコマンドと一致させること。
function buildTokens(sub: DockerSub, name: string, force: boolean, all: boolean): CmdToken[] {
  const out = [tok("docker", "cmd"), tok("builder", "cmd"), tok(sub, "sub", "sub")];
  if (sub === "inspect" && name) out.push(tok(name, "value", "name"));
  if (sub === "prune") {
    if (force) out.push(tok("-f", "flag", "force"));
    if (all) out.push(tok("--all", "flag", "all"));
  }
  return out;
}

export function useDocker() {
  const [sub, setSub] = useState<DockerSub>("du");
  const [force, setForce] = useState(DEFAULT_DOCKER_FORCE);
  const [all, setAll] = useState(DEFAULT_DOCKER_ALL);
  const [name, setName] = useState("");
  const runner = useRunner<DockerResult>();

  const nameOk = isValidBuilderName(name);
  const tokens = buildTokens(sub, name.trim(), force, all);
  const preview = tokensToString(tokens);
  const valid = sub !== "inspect" || nameOk;

  async function execute() {
    if (!valid) return;
    await runner.run(async (): Promise<DockerResult> => {
      switch (sub) {
        case "du":
          return { kind: "du", result: await api.dockerBuilderDu() };
        case "ls":
          return { kind: "ls", result: await api.dockerBuilderLs() };
        case "inspect":
          return { kind: "inspect", result: await api.dockerBuilderInspect(name.trim()) };
        case "version":
          return { kind: "version", result: await api.dockerBuilderVersion() };
        case "prune":
          return { kind: "prune", result: await api.pruneDockerBuilder(force, all) };
      }
    }, preview);
  }

  return {
    sub,
    setSub,
    force,
    setForce,
    all,
    setAll,
    name,
    setName,
    nameOk,
    valid,
    runner,
    tokens,
    preview,
    execute,
  };
}
