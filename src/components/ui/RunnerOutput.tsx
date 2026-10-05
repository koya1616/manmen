import type { ReactNode } from "react";
import type { RunState } from "../../hooks/useRunner";
import { OutputPane } from "./OutputPane";

export interface RunnerResultLike {
  success: boolean;
  exit_code: number;
  stderr: string;
}

// 全Card共通の出力ペイン定型。meta組立と stale 判定を集約する。
// result が null のときは未実行表示になる。stderr を上書きしたい場合のみ stderr を渡す。
export function RunnerOutput({
  runner,
  preview,
  emptyHint,
  result,
  stderr,
  children,
}: {
  runner: Pick<
    RunState<unknown>,
    "running" | "error" | "ranAt" | "durationMs" | "ranCommand"
  >;
  preview: string;
  emptyHint: string;
  result: RunnerResultLike | null | undefined;
  stderr?: string;
  children?: ReactNode;
}) {
  return (
    <OutputPane
      running={runner.running}
      error={runner.error}
      meta={
        result
          ? {
              success: result.success,
              exitCode: result.exit_code,
              stderr: stderr ?? result.stderr,
            }
          : null
      }
      ranAt={runner.ranAt}
      durationMs={runner.durationMs}
      ranCommand={runner.ranCommand}
      stale={runner.ranCommand !== null && runner.ranCommand !== preview}
      emptyHint={emptyHint}
    >
      {children}
    </OutputPane>
  );
}
