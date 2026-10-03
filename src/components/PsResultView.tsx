import { useTranslation } from "react-i18next";
import type { PsSnapshot } from "../types";

// ps のスナップショット表示。件数 + プロセス表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function PsResultView({ result }: { result: PsSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("psResult.processes")}</span>
          <span className="stat-value">{result.count}</span>
        </div>
      </div>

      {result.rows.length === 0 ? (
        <p className="muted">{t("psResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                {result.columns.map((column) => (
                  <th
                    key={column}
                    className={column === "%cpu" || column === "%mem" ? "num" : undefined}
                  >
                    {t(`psColumns.${column}`)}
                  </th>
                ))}
              </tr>
            </thead>
            <tbody>
              {result.rows.map((row, index) => (
                <tr key={`${row[0] ?? "row"}-${index}`}>
                  {row.map((cell, cellIndex) => (
                    <td
                      key={`${result.columns[cellIndex] ?? cellIndex}-${cell}`}
                      className={
                        result.columns[cellIndex] === "%cpu" ||
                        result.columns[cellIndex] === "%mem"
                          ? "num mono"
                          : "mono"
                      }
                    >
                      {cell}
                    </td>
                  ))}
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  );
}
