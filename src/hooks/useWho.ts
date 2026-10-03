import { api } from "../api";
import { tok, tokensToString } from "../commands/tokens";
import type { WhoSnapshot } from "../types";
import { useRunner } from "./useRunner";

const tokens = [tok("who", "cmd")];
const preview = tokensToString(tokens);

export function useWho() {
  const runner = useRunner<WhoSnapshot>();

  async function execute() {
    await runner.run(() => api.getWho(), preview);
  }

  return { runner, tokens, preview, execute };
}
