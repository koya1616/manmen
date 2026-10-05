import { useState } from "react";
import { useTranslation } from "react-i18next";
import { formatKb } from "../format";
import type { DiskUsageSnapshot } from "../types";

const INITIAL_ROWS = 50;

// du のスナップショット表示。大きい順の一覧と、全体に占める割合のバーで見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function DuResultView({ result }: { result: DiskUsageSnapshot }) {
  const { t } = useTranslation();
  const [showAll, setShowAll] = useState(false);
  const items = result.items.filter((item) => item.depth > 0);
  const base = result.total_kb ?? Math.max(1, ...items.map((item) => item.size_kb));
  const shown = showAll ? items : items.slice(0, INITIAL_ROWS);
  const prefix = result.root.endsWith("/") ? result.root : `${result.root}/`;

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("duResult.total")}</span>
          <span className="stat-value mono">
            {result.total_kb !== null ? formatKb(result.total_kb) : "—"}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("duResult.items")}</span>
          <span className="stat-value">{Math.max(0, result.total_items - 1)}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("duResult.root")}</span>
          <span className="stat-value mono stat-small">{result.root}</span>
        </div>
      </div>
      {result.timed_out ? <p className="wb-blocker">{t("duResult.timedOut")}</p> : null}
      {result.error_count > 0 ? (
        <p className="note">{t("duResult.errors", { n: result.error_count })}</p>
      ) : null}
      {result.items.length < result.total_items ? (
        <p className="note">{t("duResult.truncated", { n: result.items.length })}</p>
      ) : null}

      {items.length === 0 ? (
        <p className="muted">{t("duResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("duResult.path")}</th>
                <th className="num">{t("duResult.size")}</th>
                <th>{t("duResult.share")}</th>
              </tr>
            </thead>
            <tbody>
              {shown.map((item) => {
                const share = Math.min(100, (item.size_kb / base) * 100);
                return (
                  <tr key={item.path}>
                    <td className="mono" title={item.path}>
                      {item.path.startsWith(prefix) ? item.path.slice(prefix.length) : item.path}
                    </td>
                    <td className="num mono">{formatKb(item.size_kb)}</td>
                    <td>
                      <div className="usage-cell">
                        <div className="usage-track">
                          <div className="usage-fill" style={{ width: `${share}%` }} />
                        </div>
                        <span className="mono">{share.toFixed(1)}%</span>
                      </div>
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
      {items.length > INITIAL_ROWS ? (
        <div>
          <button type="button" className="link" onClick={() => setShowAll((v) => !v)}>
            {showAll ? t("ui.showLess") : t("ui.showAll")}
          </button>
        </div>
      ) : null}
    </>
  );
}
