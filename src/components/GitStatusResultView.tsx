import { useTranslation } from "react-i18next";
import type { GitStatusSnapshot } from "../types";

// git status の表示。ブランチ + 変更ファイル表で見せる。
// クリーンなときは文言を出す。
export function GitStatusResultView({ result }: { result: GitStatusSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("gitStatusResult.branch")}</span>
          <span className="stat-value mono">{result.branch || "—"}</span>
          {result.tracking ? (
            <span className="stat-small mono">{result.tracking}</span>
          ) : null}
        </div>
        <div className="stat">
          <span className="stat-label">{t("gitStatusResult.changes")}</span>
          <span className="stat-value">{result.count}</span>
        </div>
      </div>

      {result.files.length === 0 ? (
        <p className="muted">{t("gitStatusResult.clean")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("gitStatusResult.status")}</th>
                <th>{t("gitStatusResult.path")}</th>
              </tr>
            </thead>
            <tbody>
              {result.files.map((file, index) => (
                <tr key={`${file.xy}-${file.path}-${index}`}>
                  <td className="mono">{file.xy}</td>
                  <td className="mono">{file.path}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </>
  );
}
