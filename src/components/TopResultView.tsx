import { useTranslation } from "react-i18next";
import type { TopSnapshot } from "../types";

function fmt(value: number | null, digits = 2): string {
  return value === null ? "—" : value.toFixed(digits);
}

function CpuCell({ value }: { value: string }) {
  const cpu = Number(value);
  if (!Number.isFinite(cpu)) return value;
  return (
    <div className="cpu-cell">
      <div className="cpu-bar" style={{ width: `${Math.min(100, cpu)}%` }} />
      <span>{cpu.toFixed(1)}</span>
    </div>
  );
}

// top のスナップショット表示。生テキストではなくサマリーカード + プロセス表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function TopResultView({ result }: { result: TopSnapshot }) {
  const { t } = useTranslation();
  const s = result.summary;

  return (
    <>
      {s.timestamp ? <p className="muted">{s.timestamp}</p> : null}

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("topResult.cpu")}</span>
          <span className="stat-value">
            {t("topResult.cpuDetail", {
              user: fmt(s.cpu_user, 1),
              sys: fmt(s.cpu_sys, 1),
              idle: fmt(s.cpu_idle, 1),
            })}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.loadAvg")}</span>
          <span className="stat-value">
            {fmt(s.load_avg_1)} / {fmt(s.load_avg_5)} / {fmt(s.load_avg_15)}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.processes")}</span>
          <span className="stat-value">
            {s.processes_total ?? "—"}
            {s.processes_running !== null
              ? ` (${t("topResult.running", { n: s.processes_running })})`
              : ""}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("topResult.physmem")}</span>
          <span className="stat-value stat-small">{s.physmem || "—"}</span>
        </div>
      </div>

      {s.extra.length > 0 ? (
        <ul className="extra-lines">
          {s.extra.map((line) => (
            <li key={line}>{line}</li>
          ))}
        </ul>
      ) : null}

      {result.rows.length === 0 ? (
        <p className="muted">{t("topResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                {result.columns.map((column) => (
                  <th key={column} className={column === "%CPU" ? "num" : undefined}>
                    {column}
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
                        result.columns[cellIndex] === "%CPU" ? "num mono" : "mono"
                      }
                    >
                      {result.columns[cellIndex] === "%CPU" ? (
                        <CpuCell value={cell} />
                      ) : (
                        cell
                      )}
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
