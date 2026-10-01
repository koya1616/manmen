import { useTranslation } from "react-i18next";
import type { TopSnapshot } from "../types";

function fmt(value: number | null, digits = 2): string {
  return value === null ? "—" : value.toFixed(digits);
}

// top のスナップショット表示。生テキストではなくサマリーカード + プロセス表で見せる。
export function TopResultView({
  result,
  error,
}: {
  result: TopSnapshot | null;
  error: string | null;
}) {
  const { t } = useTranslation();

  if (error) {
    return (
      <section className="card error">
        <h3>{t("result.failed")}</h3>
        <pre>{error}</pre>
      </section>
    );
  }

  if (!result) return null;

  const s = result.summary;

  return (
    <section className={`card ${result.success ? "success" : "error"}`}>
      <h3>
        {result.success ? t("result.completed") : t("result.failed")}
        {s.timestamp ? <span className="muted"> — {s.timestamp}</span> : null}
      </h3>

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("topResult.cpu")}</span>
          <span className="stat-value">
            {t("topResult.cpuDetail", {
              user: fmt(s.cpu_user, 1),
              sys: fmt(s.cpu_sys, 1),
              idle: fmt(s.cpu_idle, 1),
            })}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.loadAvg")}</span>
          <span className="stat-value">
            {fmt(s.load_avg_1)} / {fmt(s.load_avg_5)} / {fmt(s.load_avg_15)}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.processes")}</span>
          <span className="stat-value">
            {s.processes_total ?? "—"}
            {s.processes_running !== null
              ? ` (${t("topResult.running", { n: s.processes_running })})`
              : ""}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.physmem")}</span>
          <span className="stat-value stat-small">{s.physmem || "—"}</span>
        </div>
      </div>

      {result.processes.length === 0 ? (
        <p className="muted">{t("topResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("topResult.pid")}</th>
                <th>{t("topResult.command")}</th>
                <th className="num">{t("topResult.cpuCol")}</th>
                <th>{t("topResult.time")}</th>
                <th className="num">{t("topResult.threads")}</th>
                <th>{t("topResult.mem")}</th>
                <th>{t("topResult.state")}</th>
                <th>{t("topResult.user")}</th>
              </tr>
            </thead>
            <tbody>
              {result.processes.map((p) => (
                <tr key={p.pid}>
                  <td className="mono">{p.pid}</td>
                  <td className="mono">{p.command}</td>
                  <td className="num">
                    <div className="cpu-cell">
                      <div
                        className="cpu-bar"
                        style={{ width: `${Math.min(100, p.cpu)}%` }}
                      />
                      <span className="mono">{p.cpu.toFixed(1)}</span>
                    </div>
                  </td>
                  <td className="mono">{p.time}</td>
                  <td className="num mono">{p.threads}</td>
                  <td className="mono">{p.mem}</td>
                  <td>{p.state}</td>
                  <td>{p.user}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {result.stderr ? (
        <div className="output-block">
          <h4>{t("result.error")}</h4>
          <pre>{result.stderr}</pre>
        </div>
      ) : null}

      <details className="raw-details">
        <summary>{t("topResult.raw")}</summary>
        <p className="muted">
          {t("result.exitCode")}: {result.exit_code}
        </p>
        <p className="muted">
          <code>{result.command}</code>
        </p>
      </details>
    </section>
  );
}
