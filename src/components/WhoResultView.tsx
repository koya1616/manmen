import { useTranslation } from "react-i18next";
import type { WhoSnapshot } from "../types";

// who のスナップショット表示。件数 + ユーザ表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function WhoResultView({ result }: { result: WhoSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("whoResult.users")}</span>
          <span className="stat-value">{result.count}</span>
        </div>
      </div>

      {result.users.length === 0 ? (
        <p className="muted">{t("whoResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("whoResult.user")}</th>
                <th>{t("whoResult.tty")}</th>
                <th>{t("whoResult.login")}</th>
              </tr>
            </thead>
            <tbody>
              {result.users.map((entry, index) => (
                <tr key={`${entry.user}-${entry.tty}-${index}`}>
                  <td className="mono">{entry.user}</td>
                  <td className="mono">{entry.tty}</td>
                  <td className="mono">{entry.login}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  );
}
