import { useState } from "react";
import { api } from "../api";
import {
  DU_DEFAULT_DEPTH,
  DU_DEFAULT_TIMEOUT_SECS,
  isValidDuDepth,
  isValidDuTimeout,
} from "../commands/diskOptions";
import { isValidPath } from "../commands/validate";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { DiskUsageSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useDu() {
  const [path, setPath] = useState("~");
  const [depth, setDepth] = useState(String(DU_DEFAULT_DEPTH));
  const [allFiles, setAllFiles] = useState(false);
  const [oneFs, setOneFs] = useState(true);
  const [apparent, setApparent] = useState(false);
  const [timeoutSecs, setTimeoutSecs] = useState(String(DU_DEFAULT_TIMEOUT_SECS));
  const runner = useRunner<DiskUsageSnapshot>();

  const pathOk = isValidPath(path);
  const depthOk = isValidDuDepth(depth);
  const timeoutOk = isValidDuTimeout(timeoutSecs);
  const valid = pathOk && depthOk && timeoutOk;

  const query = {
    path: path.trim(),
    depth: depthOk ? Number(depth.trim()) : DU_DEFAULT_DEPTH,
    allFiles,
    oneFs,
    apparent,
    timeoutSecs: timeoutOk ? Number(timeoutSecs.trim()) : DU_DEFAULT_TIMEOUT_SECS,
  };
  // 打ち切り時間は du のオプションではないのでコマンド表示に含めない
  const tokens = buildTokens(query);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getDu(query), preview);
  }

  return {
    path,
    setPath,
    depth,
    setDepth,
    allFiles,
    setAllFiles,
    oneFs,
    setOneFs,
    apparent,
    setApparent,
    timeoutSecs,
    setTimeoutSecs,
    runner,
    tokens,
    preview,
    valid,
    pathOk,
    depthOk,
    timeoutOk,
    execute,
  };
}

// Rust の Du::preview と同じ並びにすること (Rust 側は ~ を展開して表示する)。
function buildTokens(input: {
  path: string;
  depth: number;
  allFiles: boolean;
  oneFs: boolean;
  apparent: boolean;
}): CmdToken[] {
  const out: CmdToken[] = [tok("du", "cmd"), tok("-k", "fixed")];
  if (input.apparent) out.push(tok("-A", "flag", "apparent"));
  if (input.oneFs) out.push(tok("-x", "flag", "oneFs"));
  if (input.allFiles) out.push(tok("-a", "flag", "allFiles"));
  out.push(tok("-d", "flag", "depth"), tok(String(input.depth), "value", "depth"));
  out.push(tok(input.path || "…", input.path ? "value" : "placeholder", "path"));
  return out;
}
