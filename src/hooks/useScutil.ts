import { useState } from "react";
import { api } from "../api";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { ScutilSnapshot } from "../types";
import { useRunner } from "./useRunner";

export type ScutilSub = "dns" | "nwi" | "names";

export const SCUTIL_SUBS: ScutilSub[] = ["dns", "nwi", "names"];

// Rust の scutil.rs が組み立てるコマンドと一致させること。
function buildTokens(sub: ScutilSub): CmdToken[] {
  if (sub === "names") {
    return [
      tok("scutil", "cmd"),
      tok("--get", "sub", "sub"),
      tok("ComputerName/LocalHostName/HostName", "value"),
    ];
  }
  return [tok("scutil", "cmd"), tok(`--${sub}`, "sub", "sub")];
}

export function useScutil() {
  const [sub, setSub] = useState<ScutilSub>("dns");
  const runner = useRunner<ScutilSnapshot>();

  const tokens = buildTokens(sub);
  const preview = tokensToString(tokens);

  async function execute() {
    await runner.run(() => api.getScutil(sub), preview);
  }

  return { sub, setSub, runner, tokens, preview, execute };
}
