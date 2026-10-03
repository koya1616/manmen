import { useTranslation } from "react-i18next";
import type { WSnapshot } from "../types";

// w のスナップショット表示。ヘッダ + 件数 + ユーザ表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function WResultView({ result }: { result: WSnapshot }) {
  const { t } = useTranslation();
  const loads = [result.summary.load_avg_1, result.summary.load_avg_5, result.summary.load_avg_15];
  const hasLoads = loads.every((value) => value !== null);

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("wResult.users")}</span>
          <span className="stat-value">{result.count}</span>
        </div>
        {hasLoads ? (
          <div className="stat">
            <span className="stat-label">{t("wResult.loadAvg")}</span>
            <span className="stat-value mono">
              {loads.map((value) => value?.toFixed(2)).join(" / ")}
            </span>
          </div>
        ) : null}
      </div>
      {result.summary.headline ? (
        <pre className="term">{result.summary.headline}</pre>
      ) : null}

      {result.users.length === 0 ? (
        <p className="muted">{t("wResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("wResult.user")}</th>
                <th>{t("wResult.tty")}</th>
                <th>{t("wResult.from")}</th>
                <th>{t("wResult.login")}</th>
                <th>{t("wResult.idle")}</th>
                <th>{t("wResult.what")}</th>
              </tr>
            </thead>
            <tbody>
              {result.users.map((entry, index) => (
                <tr key={`${entry.user}-${entry.tty}-${index}`}>
                  <td className="mono">{entry.user}</td>
                  <td className="mono">{entry.tty}</td>
                  <td className="mono">{entry.from}</td>
                  <td className="mono">{entry.login}</td>
                  <td className="mono">{entry.idle}</td>
                  <td className="mono">{entry.what}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  );
}
