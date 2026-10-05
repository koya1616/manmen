import { useTranslation } from "react-i18next";
import type { KillSnapshot } from "../types";

// kill のスナップショット表示。PID ごとの送信結果を表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function KillResultView({ result }: { result: KillSnapshot }) {
  const { t } = useTranslation();
  const sent = result.results.filter((r) => r.ok).length;

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("killResult.signal")}</span>
          <span className="stat-value mono">SIG{result.signal}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("killResult.sent")}</span>
          <span className="stat-value">
            {sent} / {result.results.length}
          </span>
        </div>
      </div>
      <div className="table-wrap">
        <table className="top-table">
          <thead>
            <tr>
              <th>PID</th>
              <th>{t("killResult.status")}</th>
              <th>{t("killResult.error")}</th>
            </tr>
          </thead>
          <tbody>
            {result.results.map((r) => (
              <tr key={r.pid}>
                <td className="mono">{r.pid}</td>
                <td>{r.ok ? t("killResult.ok") : t("killResult.failed")}</td>
                <td className="mono">{r.error ? errorLabel(r.error, t) : "—"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <p className="note">{t("killResult.hint")}</p>
    </>
  );
}

function errorLabel(error: string, t: (key: string) => string): string {
  if (error === "No such process") return `${t("killResult.noSuchProcess")} (${error})`;
  if (error === "Operation not permitted") return `${t("killResult.notPermitted")} (${error})`;
  return error;
}
