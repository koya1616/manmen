import { useTranslation } from "react-i18next";
import type { VersionSnapshot } from "../types";

// `docker builder version` の分割表示。1行を package / version / commit で見せる。
export function DockerVersionResultView({
  result,
  error,
}: {
  result: VersionSnapshot | null;
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

  const v = result.version;

  return (
    <section className={`card ${result.success ? "success" : "error"}`}>
      <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("dockerVersion.version")}</span>
          <span className="stat-value">{v.version || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerVersion.package")}</span>
          <span className="stat-value stat-small">{v.package || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerVersion.commit")}</span>
          <span className="stat-value stat-small mono">
            {v.commit ? v.commit.slice(0, 12) : "—"}
          </span>
        </div>
      </div>

      {result.stderr ? (
        <div className="output-block">
          <h4>{t("result.error")}</h4>
          <pre>{result.stderr}</pre>
        </div>
      ) : null}

      <details className="raw-details">
        <summary>{t("dockerVersion.raw")}</summary>
        <p className="muted">
          <code>{result.raw || t("result.noOutput")}</code>
        </p>
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
