import { useMemo } from "react";
import { tok, tokensToString } from "../commands/tokens";
import { useRunner } from "./useRunner";

// 引数なし単発コマンド用 (w / who / whoami 等)。tokens/preview/execute の定型を集約する。
export function useSimpleCommand<T>(cmd: string, fetch: () => Promise<T>) {
  const runner = useRunner<T>();
  const tokens = useMemo(() => [tok(cmd, "cmd")], [cmd]);
  const preview = useMemo(() => tokensToString(tokens), [tokens]);

  async function execute() {
    await runner.run(fetch, preview);
  }

  return { runner, tokens, preview, execute };
}
