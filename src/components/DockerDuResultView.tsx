import { useTranslation } from "react-i18next";
import type { DuSnapshot } from "../types";

// `docker builder du` の表 + 集計表示。生テキストではなく表で見せる。
export function DockerDuResultView({
  result,
  error,
}: {
  result: DuSnapshot | null;
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
      <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("dockerDu.total")}</span>
          <span className="stat-value">{s.total || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerDu.reclaimable")}</span>
          <span className="stat-value">{s.reclaimable || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerDu.shared")}</span>
          <span className="stat-value">{s.shared || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerDu.private")}</span>
          <span className="stat-value">{s.private || "—"}</span>
        </div>
      </div>

      {result.entries.length === 0 ? (
        <p className="muted">{t("dockerDu.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("dockerDu.id")}</th>
                <th>{t("dockerDu.size")}</th>
                <th>{t("dockerDu.reclaimableCol")}</th>
                <th>{t("dockerDu.sharedCol")}</th>
                <th>{t("dockerDu.lastAccessed")}</th>
              </tr>
            </thead>
            <tbody>
              {result.entries.map((entry) => (
                <tr key={entry.id}>
                  <td className="mono">{entry.id.slice(0, 12)}</td>
                  <td className="num mono">{entry.size}</td>
                  <td>{entry.reclaimable ? "✓" : "—"}</td>
                  <td>{entry.shared ? "✓" : "—"}</td>
                  <td className="mono">{entry.last_accessed}</td>
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
        <summary>{t("dockerDu.raw")}</summary>
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
