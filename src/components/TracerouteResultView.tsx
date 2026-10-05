import { useTranslation } from "react-i18next";
import type { TracerouteHop, TracerouteSnapshot } from "../types";

// traceroute のスナップショット表示。概要 + ホップ表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function TracerouteResultView({ result }: { result: TracerouteSnapshot }) {
  const { t } = useTranslation();
  const maxAvg = Math.max(0, ...result.hops.map((hop) => hopAvg(hop) ?? 0));

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("tracerouteResult.reached")}</span>
          <span className="stat-value">
            {result.reached ? t("tracerouteResult.yes") : t("tracerouteResult.no")}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("tracerouteResult.hops")}</span>
          <span className="stat-value">
            {result.hops.length}
            {result.max_hops > 0 ? ` / ${result.max_hops}` : ""}
          </span>
        </div>
        {result.resolved_ip ? (
          <div className="stat">
            <span className="stat-label">{t("tracerouteResult.target")}</span>
            <span className="stat-value mono stat-small">
              {result.target} ({result.resolved_ip})
            </span>
          </div>
        ) : null}
      </div>

      {result.hops.length === 0 ? (
        <p className="muted">{t("tracerouteResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("tracerouteResult.hop")}</th>
                <th>{t("tracerouteResult.responder")}</th>
                <th className="num">{t("tracerouteResult.avg")}</th>
                <th>{t("tracerouteResult.rtts")}</th>
                <th className="num">{t("tracerouteResult.lost")}</th>
              </tr>
            </thead>
            <tbody>
              {result.hops.map((hop) => {
                const avg = hopAvg(hop);
                const probes = hop.timeouts + hop.responders.reduce((n, r) => n + r.rtts_ms.length, 0);
                return (
                  <tr key={hop.ttl}>
                    <td className="mono">{hop.ttl}</td>
                    <td className="mono">
                      {hop.responders.length === 0 ? (
                        <span className="muted">{t("tracerouteResult.noReply")}</span>
                      ) : (
                        hop.responders.map((r) => (
                          <div key={r.ip || r.host}>
                            {r.asn ? <span className="muted">[{r.asn}] </span> : null}
                            {r.host}
                            {r.host !== r.ip ? <span className="muted"> ({r.ip})</span> : null}
                            {r.annotations.length > 0 ? (
                              <span className="opt-error"> {[...new Set(r.annotations)].join(" ")}</span>
                            ) : null}
                          </div>
                        ))
                      )}
                    </td>
                    <td className="num mono">
                      {avg !== null ? (
                        <div className="cpu-cell">
                          <span>{avg.toFixed(1)} ms</span>
                          <div
                            className="cpu-bar"
                            style={{ width: `${maxAvg > 0 ? (avg / maxAvg) * 100 : 0}%` }}
                          />
                        </div>
                      ) : (
                        "—"
                      )}
                    </td>
                    <td className="mono muted">
                      {[
                        ...hop.responders.flatMap((r) => r.rtts_ms.map((ms) => ms.toFixed(1))),
                        ...Array<string>(hop.timeouts).fill("*"),
                      ].join("  ")}
                    </td>
                    <td className="num mono">
                      {hop.timeouts} / {probes}
                    </td>
                  </tr>
                );
              })}
            </tbody>
          </table>
        </div>
      )}
      <p className="note">{t("tracerouteResult.hint")}</p>
    </>
  );
}

function hopAvg(hop: TracerouteHop): number | null {
  const rtts = hop.responders.flatMap((r) => r.rtts_ms);
  if (rtts.length === 0) return null;
  return rtts.reduce((a, b) => a + b, 0) / rtts.length;
}
