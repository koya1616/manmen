import { useTranslation } from "react-i18next";
import type { SystemInfoSnapshot } from "../types";

function formatBytes(n: number): string {
  if (!n) return "—";
  const gb = n / 1024 / 1024 / 1024;
  return `${gb.toFixed(1)} GB`;
}

export function DockerSystemInfoResultView({ result }: { result: SystemInfoSnapshot }) {
  const { t } = useTranslation();
  const i = result.info;

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("dockerSystemInfo.containers")}</span>
          <span className="stat-value">{i.containers}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerSystemInfo.running")}</span>
          <span className="stat-value">{i.containers_running}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerSystemInfo.images")}</span>
          <span className="stat-value">{i.images}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerSystemInfo.version")}</span>
          <span className="stat-value">{i.server_version || "—"}</span>
        </div>
      </div>
      <div className="table-wrap">
        <table className="top-table">
          <tbody>
            <tr>
              <td className="mono">{t("dockerSystemInfo.os")}</td>
              <td className="mono">{i.operating_system || "—"}</td>
            </tr>
            <tr>
              <td className="mono">{t("dockerSystemInfo.arch")}</td>
              <td className="mono">{i.architecture || "—"}</td>
            </tr>
            <tr>
              <td className="mono">{t("dockerSystemInfo.cpus")}</td>
              <td className="num mono">{i.ncpu}</td>
            </tr>
            <tr>
              <td className="mono">{t("dockerSystemInfo.memory")}</td>
              <td className="num mono">{formatBytes(i.mem_total)}</td>
            </tr>
            <tr>
              <td className="mono">{t("dockerSystemInfo.driver")}</td>
              <td className="mono">{i.driver || "—"}</td>
            </tr>
            <tr>
              <td className="mono">{t("dockerSystemInfo.kernel")}</td>
              <td className="mono">{i.kernel_version || "—"}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </>
  );
}
