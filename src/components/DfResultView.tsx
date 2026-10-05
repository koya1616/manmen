import { useTranslation } from "react-i18next";
import { formatKb } from "../format";
import type { DfRow, DfSnapshot } from "../types";

// df のスナップショット表示。ボリュームごとの使用率をバーで見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function DfResultView({ result, hideSystem }: { result: DfSnapshot; hideSystem: boolean }) {
  const { t } = useTranslation();
  const rows = hideSystem ? result.rows.filter((row) => !isSystemVolume(row)) : result.rows;
  const hidden = result.rows.length - rows.length;

  return (
    <>
      {rows.length === 0 ? (
        <p className="muted">{t("dfResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("dfResult.mount")}</th>
                <th className="num">{t("dfResult.size")}</th>
                <th className="num">{t("dfResult.used")}</th>
                <th className="num">{t("dfResult.avail")}</th>
                <th>{t("dfResult.capacity")}</th>
                <th>{t("dfResult.type")}</th>
                <th>{t("dfResult.filesystem")}</th>
                <th className="num">{t("dfResult.inodes")}</th>
              </tr>
            </thead>
            <tbody>
              {rows.map((row) => (
                <tr key={`${row.filesystem}-${row.mount}`}>
                  <td className="mono">{row.mount}</td>
                  <td className="num mono">{formatKb(row.size_kb)}</td>
                  <td className="num mono">{formatKb(row.used_kb)}</td>
                  <td className="num mono">{formatKb(row.avail_kb)}</td>
                  <td>
                    <div className="usage-cell">
                      <div className="usage-track">
                        <div
                          className={`usage-fill ${usageLevel(row.capacity_percent)}`}
                          style={{ width: `${Math.min(100, row.capacity_percent)}%` }}
                        />
                      </div>
                      <span className="mono">{row.capacity_percent}%</span>
                    </div>
                  </td>
                  <td className="mono">{row.fs_type}</td>
                  <td className="mono muted">{row.filesystem}</td>
                  <td className="num mono">{row.inode_percent}%</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
      {hidden > 0 ? <p className="note">{t("dfResult.hidden", { n: hidden })}</p> : null}
      <p className="note">{t("dfResult.hint")}</p>
    </>
  );
}

// 普段は気にしなくてよい macOS の内部ボリューム。/System/Volumes/Data は利用者のデータなので残す。
function isSystemVolume(row: DfRow): boolean {
  if (row.fs_type === "devfs" || row.fs_type === "autofs") return true;
  if (row.mount.startsWith("/System/Volumes/") && row.mount !== "/System/Volumes/Data") return true;
  return row.mount.startsWith("/Library/Developer/CoreSimulator/");
}

function usageLevel(percent: number): string {
  if (percent >= 95) return "is-danger";
  if (percent >= 85) return "is-warn";
  return "";
}
