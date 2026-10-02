import { useTranslation } from "react-i18next";
import type { SystemDfSnapshot } from "../types";

export function DockerSystemDfResultView({ result }: { result: SystemDfSnapshot }) {
  const { t } = useTranslation();

  if (result.entries.length === 0) {
    return <p className="muted">{t("dockerSystemDf.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("dockerSystemDf.type")}</th>
            <th>{t("dockerSystemDf.total")}</th>
            <th>{t("dockerSystemDf.active")}</th>
            <th>{t("dockerSystemDf.size")}</th>
            <th>{t("dockerSystemDf.reclaimable")}</th>
          </tr>
        </thead>
        <tbody>
          {result.entries.map((e) => (
            <tr key={e.dtype}>
              <td className="mono">{e.dtype}</td>
              <td className="num mono">{e.total}</td>
              <td className="num mono">{e.active}</td>
              <td className="num mono">{e.size}</td>
              <td className="mono">{e.reclaimable}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
