import { useTranslation } from "react-i18next";
import type { GitLogSnapshot } from "../types";

// git log の表示。直近20件を表で見せる。
export function GitLogResultView({ result }: { result: GitLogSnapshot }) {
  const { t } = useTranslation();

  if (result.commits.length === 0) {
    return <p className="muted">{t("gitLogResult.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("gitLogResult.hash")}</th>
            <th>{t("gitLogResult.author")}</th>
            <th>{t("gitLogResult.date")}</th>
            <th>{t("gitLogResult.subject")}</th>
          </tr>
        </thead>
        <tbody>
          {result.commits.map((commit) => (
            <tr key={commit.hash}>
              <td className="mono">{commit.hash}</td>
              <td className="mono">{commit.author}</td>
              <td className="mono">{commit.date}</td>
              <td className="mono">{commit.subject}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
