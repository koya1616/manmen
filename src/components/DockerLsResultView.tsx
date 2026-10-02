import { useTranslation } from "react-i18next";
import type { LsSnapshot } from "../types";

// `docker builder ls` の階層表示。builder ごとに見出し + ノード表で見せる。
export function DockerLsResultView({ result }: { result: LsSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      {result.builders.length === 0 ? (
        <p className="muted">{t("dockerLs.noData")}</p>
      ) : (
        result.builders.map((builder) => (
          <div key={builder.name} className="output-block">
            <h4>
              <code>{builder.name}</code>
              {builder.is_current ? <span> {t("dockerLs.current")}</span> : null}
              {builder.driver ? (
                <span className="muted"> — {builder.driver}</span>
              ) : null}
            </h4>
            {builder.nodes.length === 0 ? (
              <p className="muted">{t("dockerLs.noNodes")}</p>
            ) : (
              <div className="table-wrap">
                <table className="top-table">
                  <thead>
                    <tr>
                      <th>{t("dockerLs.node")}</th>
                      <th>{t("dockerLs.status")}</th>
                      <th>{t("dockerLs.buildkit")}</th>
                      <th>{t("dockerLs.platforms")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {builder.nodes.map((node) => (
                      <tr key={`${builder.name}/${node.name}`}>
                        <td className="mono">{node.name}</td>
                        <td className="mono">{node.status}</td>
                        <td className="mono">{node.buildkit}</td>
                        <td className="mono">{node.platforms}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </div>
        ))
      )}
    </>
  );
}
