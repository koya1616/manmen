import { useState } from "react";

export interface RunState<T> {
  result: T | null;
  error: string | null;
  running: boolean;
  ranAt: number | null;
  durationMs: number | null;
  ranCommand: string | null;
}

// 全コマンド共通の実行状態。再実行中も直前の出力は残し、出力ペインで薄く見せる。
export function useRunner<T>() {
  const [state, setState] = useState<RunState<T>>({
    result: null,
    error: null,
    running: false,
    ranAt: null,
    durationMs: null,
    ranCommand: null,
  });

  async function run(task: () => Promise<T>, command: string): Promise<T | null> {
    const started = performance.now();
    setState((prev) => ({ ...prev, running: true, error: null }));
    try {
      const result = await task();
      setState({
        result,
        error: null,
        running: false,
        ranAt: Date.now(),
        durationMs: performance.now() - started,
        ranCommand: command,
      });
      return result;
    } catch (e) {
      setState({
        result: null,
        error: String(e),
        running: false,
        ranAt: Date.now(),
        durationMs: performance.now() - started,
        ranCommand: command,
      });
      return null;
    }
  }

  return { ...state, run };
}
