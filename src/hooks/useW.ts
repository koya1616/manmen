import { api } from "../api";
import { tok, tokensToString } from "../commands/tokens";
import type { WSnapshot } from "../types";
import { useRunner } from "./useRunner";

const tokens = [tok("w", "cmd")];
const preview = tokensToString(tokens);

export function useW() {
  const runner = useRunner<WSnapshot>();

  async function execute() {
    await runner.run(() => api.getW(), preview);
  }

  return { runner, tokens, preview, execute };
}
