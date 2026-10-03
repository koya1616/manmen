import { api } from "../api";
import { tok, tokensToString } from "../commands/tokens";
import type { CommandResult } from "../types";
import { useRunner } from "./useRunner";

const tokens = [tok("whoami", "cmd")];
const preview = tokensToString(tokens);

export function useWhoami() {
  const runner = useRunner<CommandResult>();

  async function execute() {
    await runner.run(() => api.getWhoami(), preview);
  }

  return { runner, tokens, preview, execute };
}
