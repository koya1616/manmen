import { useTranslation } from "react-i18next";
import type { DigSnapshot } from "../types";

// dig の問い合わせ結果表示。ANSWER を表で見せ、統計は上部のカードに出す。
// 生テキスト全体 (raw) は折りたたみで確認できる。
export function DigResultView({ result }: { result: DigSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      {(result.query_time || result.server || result.when || result.msg_size) && (
        <div className="stat-grid">
          {result.query_time ? (
            <div className="stat">
              <span className="stat-label">{t("digResult.queryTime")}</span>
              <span className="stat-value">{result.query_time}</span>
            </div>
          ) : null}
          {result.server ? (
            <div className="stat">
              <span className="stat-label">{t("digResult.server")}</span>
              <span className="stat-value stat-small mono">{result.server}</span>
            </div>
          ) : null}
          {result.when ? (
            <div className="stat">
              <span className="stat-label">{t("digResult.when")}</span>
              <span className="stat-value stat-small">{result.when}</span>
            </div>
          ) : null}
          {result.msg_size ? (
            <div className="stat">
              <span className="stat-label">{t("digResult.msgSize")}</span>
              <span className="stat-value">{result.msg_size}</span>
            </div>
          ) : null}
        </div>
      )}

      {result.answers.length === 0 ? (
        <p className="muted">{t("digResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("digResult.name")}</th>
                <th>{t("digResult.ttl")}</th>
                <th>{t("digResult.class")}</th>
                <th>{t("digResult.type")}</th>
                <th>{t("digResult.value")}</th>
              </tr>
            </thead>
            <tbody>
              {result.answers.map((answer, index) => (
                <tr key={`${answer.name}-${answer.value}-${index}`}>
                  <td className="mono">{answer.name}</td>
                  <td className="num mono">{answer.ttl ?? "—"}</td>
                  <td className="mono">{answer.class || "—"}</td>
                  <td className="mono">{answer.dtype}</td>
                  <td className="mono">{answer.value}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      {result.raw ? (
        <details className="about">
          <summary>{t("digResult.raw")}</summary>
          <pre className="term">{result.raw}</pre>
        </details>
      ) : null}
    </>
  );
}
