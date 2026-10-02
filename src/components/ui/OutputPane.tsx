import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";

export interface OutputMeta {
  success: boolean;
  exitCode: number;
  stderr: string;
}

// 全コマンド共通の出力ペイン。状態・終了コード・実行コマンドを常に上部に出し、
// 本文はコマンドごとの表示 (children) に任せる。
export function OutputPane({
  running,
  error,
  meta,
  ranAt,
  durationMs,
  ranCommand,
  stale,
  emptyHint,
  children,
}: {
  running: boolean;
  error: string | null;
  meta: OutputMeta | null;
  ranAt: number | null;
  durationMs: number | null;
  ranCommand: string | null;
  stale: boolean;
  emptyHint: string;
  children?: ReactNode;
}) {
  const { t } = useTranslation();

  const status = running
    ? "running"
    : error || (meta && !meta.success)
      ? "failed"
      : meta
        ? "ok"
        : "idle";

  return (
    <section className={`out out-${status}`}>
      <header className="out-head">
        <div className="pane-label">{t("ui.output")}</div>
        <span className={`badge badge-${status}`}>{t(`ui.status.${status}`)}</span>
        {meta ? (
          <span className="out-meta">
            {t("result.exitCode")} {meta.exitCode}
          </span>
        ) : null}
        {ranAt ? (
          <span className="out-meta">
            {new Date(ranAt).toLocaleTimeString()}
            {durationMs !== null ? ` · ${(durationMs / 1000).toFixed(2)}s` : ""}
          </span>
        ) : null}
        {stale && !running ? <span className="badge badge-stale">{t("ui.stale")}</span> : null}
      </header>
      {ranCommand ? (
        <code className="out-cmd">
          <span className="cmd-prompt">$</span> {ranCommand}
        </code>
      ) : null}
      {running ? <div className="out-progress" /> : null}

      <div className={`out-body ${running ? "is-dim" : ""}`}>
        {error ? (
          <pre className="term term-error">{error}</pre>
        ) : meta || children ? (
          <>
            {children}
            {meta?.stderr ? (
              <div className="out-block">
                <h4>{t("result.error")}</h4>
                <pre className="term term-error">{meta.stderr}</pre>
              </div>
            ) : null}
          </>
        ) : running ? null : (
          <div className="out-empty">
            <div className="out-empty-icon" aria-hidden>
              ▶
            </div>
            <p>{emptyHint}</p>
            <p className="muted">
              <kbd>⌘</kbd> + <kbd>↵</kbd> {t("ui.shortcutRun")}
            </p>
          </div>
        )}
      </div>
    </section>
  );
}
