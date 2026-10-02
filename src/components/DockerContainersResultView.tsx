import { useTranslation } from "react-i18next";
import type { ContainerSnapshot } from "../types";

// `docker container ls -a --format json` の表表示。
export function DockerContainersResultView({ result }: { result: ContainerSnapshot }) {
  const { t } = useTranslation();

  if (result.containers.length === 0) {
    return <p className="muted">{t("dockerContainers.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("dockerContainers.name")}</th>
            <th>{t("dockerContainers.image")}</th>
            <th>{t("dockerContainers.status")}</th>
            <th>{t("dockerContainers.ports")}</th>
          </tr>
        </thead>
        <tbody>
          {result.containers.map((c) => (
            <tr key={c.id}>
              <td className="mono">{c.names}</td>
              <td className="mono">{c.image}</td>
              <td className="mono">
                {c.status}
                {c.state ? <span className="muted"> ({c.state})</span> : null}
              </td>
              <td className="mono">{c.ports || "—"}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
