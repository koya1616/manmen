import { useTranslation } from "react-i18next";
import type { GitBranchesSnapshot } from "../types";

// git branch -a の表示。現行ブランチに印を付ける。
export function GitBranchesResultView({ result }: { result: GitBranchesSnapshot }) {
  const { t } = useTranslation();

  if (result.branches.length === 0) {
    return <p className="muted">{t("gitBranchesResult.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("gitBranchesResult.current")}</th>
            <th>{t("gitBranchesResult.name")}</th>
            <th>{t("gitBranchesResult.remote")}</th>
          </tr>
        </thead>
        <tbody>
          {result.branches.map((branch) => (
            <tr key={`${branch.remote ? "r" : "l"}-${branch.name}`}>
              <td className="mono">{branch.current ? "*" : ""}</td>
              <td className="mono">{branch.name}</td>
              <td className="mono">
                {branch.remote ? t("gitBranchesResult.yes") : ""}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
