import { useTranslation } from "react-i18next";
import type { GitRemotesSnapshot } from "../types";

// git remote -v の表示。
export function GitRemotesResultView({ result }: { result: GitRemotesSnapshot }) {
  const { t } = useTranslation();

  if (result.remotes.length === 0) {
    return <p className="muted">{t("gitRemotesResult.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("gitRemotesResult.name")}</th>
            <th>{t("gitRemotesResult.url")}</th>
            <th>{t("gitRemotesResult.kind")}</th>
          </tr>
        </thead>
        <tbody>
          {result.remotes.map((remote, index) => (
            <tr key={`${remote.name}-${remote.kind}-${index}`}>
              <td className="mono">{remote.name}</td>
              <td className="mono">{remote.url}</td>
              <td className="mono">{remote.kind}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
