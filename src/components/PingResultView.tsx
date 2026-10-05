import { useTranslation } from "react-i18next";
import type { PingSnapshot } from "../types";

// ping のスナップショット表示。集計 + 応答表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function PingResultView({ result }: { result: PingSnapshot }) {
  const { t } = useTranslation();
  const { stats } = result;

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("pingResult.transmitted")}</span>
          <span className="stat-value">{stats.transmitted}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("pingResult.received")}</span>
          <span className="stat-value">{stats.received}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("pingResult.loss")}</span>
          <span className="stat-value mono">{stats.loss_percent.toFixed(1)}%</span>
        </div>
        {stats.avg_ms !== null ? (
          <div className="stat">
            <span className="stat-label">{t("pingResult.avg")}</span>
            <span className="stat-value mono">{stats.avg_ms.toFixed(3)} ms</span>
          </div>
        ) : null}
        {stats.min_ms !== null && stats.max_ms !== null ? (
          <div className="stat">
            <span className="stat-label">{t("pingResult.minMax")}</span>
            <span className="stat-value mono">
              {stats.min_ms.toFixed(3)} / {stats.max_ms.toFixed(3)} ms
            </span>
          </div>
        ) : null}
      </div>
      {result.resolved_ip ? (
        <p className="muted mono">
          {result.target} ({result.resolved_ip})
        </p>
      ) : null}

      {result.replies.length === 0 ? (
        <p className="muted">{t("pingResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>seq</th>
                <th>{t("pingResult.from")}</th>
                <th>{t("pingResult.ttl")}</th>
                <th>{t("pingResult.time")}</th>
                <th>{t("pingResult.status")}</th>
              </tr>
            </thead>
            <tbody>
              {result.replies.map((reply) => (
                <tr key={reply.seq}>
                  <td className="mono">{reply.seq}</td>
                  <td className="mono">{reply.timeout ? "—" : reply.from}</td>
                  <td className="mono">{reply.ttl ?? "—"}</td>
                  <td className="mono">
                    {reply.time_ms !== null ? `${reply.time_ms.toFixed(3)} ms` : "—"}
                  </td>
                  <td className="mono">
                    {reply.timeout ? t("pingResult.timeout") : t("pingResult.ok")}
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  );
}
