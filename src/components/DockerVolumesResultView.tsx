import { useTranslation } from "react-i18next";
import type { VolumeSnapshot } from "../types";

export function DockerVolumesResultView({ result }: { result: VolumeSnapshot }) {
  const { t } = useTranslation();

  if (result.volumes.length === 0) {
    return <p className="muted">{t("dockerVolumes.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("dockerVolumes.name")}</th>
            <th>{t("dockerVolumes.driver")}</th>
            <th>{t("dockerVolumes.scope")}</th>
          </tr>
        </thead>
        <tbody>
          {result.volumes.map((v) => (
            <tr key={v.name}>
              <td className="mono">{v.name}</td>
              <td className="mono">{v.driver}</td>
              <td className="mono">{v.scope}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
