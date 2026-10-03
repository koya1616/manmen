import { useTranslation } from "react-i18next";
import type { LsofSnapshot } from "../types";

// lsof の結果表示。件数 + オープンファイル表で見せる。
// 列は固定 (command / pid / user / fd / type / device / size / node / name)。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function LsofResultView({ result }: { result: LsofSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("lsofResult.entries")}</span>
          <span className="stat-value">{result.count}</span>
        </div>
      </div>

      {result.rows.length === 0 ? (
        <p className="muted">{t("lsofResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                {result.columns.map((column) => (
                  <th key={column}>{t(`lsofColumns.${column}`)}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {result.rows.map((row, index) => (
                <tr key={`${row[1] ?? "row"}-${index}`}>
                  {row.map((cell, cellIndex) => (
                    <td
                      key={`${result.columns[cellIndex] ?? cellIndex}-${cell}`}
                      className="mono"
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
