import { useTranslation } from "react-i18next";
import { SSH_FEATURED_KEYS } from "../commands/sshOptions";
import type { SshSnapshot } from "../types";

// ssh の結果表示。config モードは注目キー + 全件表、test モードは疎通の成否。
// 生テキスト全体 (raw) は折りたたみで確認できる。
export function SshResultView({ result }: { result: SshSnapshot }) {
  const { t } = useTranslation();

  if (result.mode === "test") {
    return (
      <>
        <p className="muted">
          {result.success ? t("sshResult.testOk") : t("sshResult.testNg")}
        </p>
        {result.stdout ? <pre className="term">{result.stdout}</pre> : null}
        {!result.success && !result.stderr ? (
          <p className="muted">{t("sshResult.noData")}</p>
        ) : null}
      </>
    );
  }

  const featured = result.entries.filter((entry) =>
    SSH_FEATURED_KEYS.includes(entry.key.toLowerCase()),
  );
  const rest = result.entries.filter(
    (entry) => !SSH_FEATURED_KEYS.includes(entry.key.toLowerCase()),
  );

  return (
    <>
      {featured.length > 0 ? (
        <div className="stat-grid">
          {featured.slice(0, 4).map((entry) => (
            <div className="stat" key={entry.key}>
              <span className="stat-label">{entry.key}</span>
              <span className="stat-value stat-small mono">{entry.value || "—"}</span>
            </div>
          ))}
        </div>
      ) : null}

      {result.entries.length === 0 ? (
        <p className="muted">{t("sshResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("sshResult.key")}</th>
                <th>{t("sshResult.value")}</th>
              </tr>
            </thead>
            <tbody>
              {[...featured, ...rest].map((entry, index) => (
                <tr key={`${entry.key}-${index}`}>
                  <td className="mono">{entry.key}</td>
                  <td className="mono">{entry.value || "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {result.raw ? (
        <details className="about">
          <summary>{t("sshResult.raw")}</summary>
          <pre className="term">{result.raw}</pre>
        </details>
      ) : null}
    </>
  );
}
