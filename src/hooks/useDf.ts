import { useState } from "react";
import { api } from "../api";
import { isValidPath } from "../commands/validate";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { DfSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useDf() {
  const [path, setPath] = useState("");
  const [all, setAll] = useState(false);
  const [local, setLocal] = useState(false);
  const [fsType, setFsType] = useState("");
  // 表示だけの絞り込み (コマンドには影響しない)
  const [hideSystem, setHideSystem] = useState(true);
  const runner = useRunner<DfSnapshot>();

  const pathOk = path.trim() === "" || isValidPath(path);
  const valid = pathOk;

  const query = { path: path.trim(), all, local, fsType };
  const tokens = buildTokens(query);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getDf(query), preview);
  }

  return {
    path,
    setPath,
    all,
    setAll,
    local,
    setLocal,
    fsType,
    setFsType,
    hideSystem,
    setHideSystem,
    runner,
    tokens,
    preview,
    valid,
    pathOk,
    execute,
  };
}

// Rust の Df::preview と同じ並びにすること。
function buildTokens(input: {
  path: string;
  all: boolean;
  local: boolean;
  fsType: string;
}): CmdToken[] {
  const out: CmdToken[] = [
    tok("df", "cmd"),
    tok("-k", "fixed"),
    tok("-Y", "fixed"),
    tok("-i", "fixed"),
  ];
  if (input.all) out.push(tok("-a", "flag", "all"));
  if (input.local) out.push(tok("-l", "flag", "local"));
  if (input.fsType) out.push(tok("-T", "flag", "fsType"), tok(input.fsType, "value", "fsType"));
  if (input.path) out.push(tok(input.path, "value", "path"));
  return out;
}
