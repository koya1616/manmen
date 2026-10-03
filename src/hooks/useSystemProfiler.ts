import { useState } from "react";
import { api } from "../api";
import {
  DEFAULT_SYSTEM_PROFILER_TYPE,
  SYSTEM_PROFILER_TYPES,
} from "../commands/systemProfilerTypes";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { SystemProfilerSnapshot } from "../types";
import { useRunner } from "./useRunner";

// Rust の system_profiler.rs が組み立てるコマンドと一致させること。
function buildTokens(dataType: string): CmdToken[] {
  return [
    tok("system_profiler", "cmd"),
    tok("-json", "flag"),
    tok(dataType, "value", "dataType"),
  ];
}

export function useSystemProfiler() {
  const [dataType, setDataType] = useState(DEFAULT_SYSTEM_PROFILER_TYPE);
  const runner = useRunner<SystemProfilerSnapshot>();

  const valid = SYSTEM_PROFILER_TYPES.includes(dataType);
  const tokens = buildTokens(dataType);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getSystemProfiler(dataType), preview);
  }

  return { dataType, setDataType, runner, tokens, preview, valid, execute };
}
