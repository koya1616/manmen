import { useTranslation } from "react-i18next";
import type { NetworkSnapshot } from "../types";

export function DockerNetworksResultView({ result }: { result: NetworkSnapshot }) {
  const { t } = useTranslation();

  if (result.networks.length === 0) {
    return <p className="muted">{t("dockerNetworks.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("dockerNetworks.name")}</th>
            <th>{t("dockerNetworks.driver")}</th>
            <th>{t("dockerNetworks.scope")}</th>
          </tr>
        </thead>
        <tbody>
          {result.networks.map((n) => (
            <tr key={n.id}>
              <td className="mono">{n.name}</td>
              <td className="mono">{n.driver}</td>
              <td className="mono">{n.scope}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
