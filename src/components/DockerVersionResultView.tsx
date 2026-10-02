import { useTranslation } from "react-i18next";
import type { VersionSnapshot } from "../types";

// `docker builder version` の分割表示。1行を package / version / commit で見せる。
export function DockerVersionResultView({ result }: { result: VersionSnapshot }) {
  const { t } = useTranslation();

  const v = result.version;

  return (
    <>
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
      {result.raw ? <pre className="term">{result.raw}</pre> : null}
    </>
  );
}
