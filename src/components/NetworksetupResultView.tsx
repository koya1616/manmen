import { useTranslation } from "react-i18next";
import type {
  NetworksetupInfoSnapshot,
  NetworksetupLocationsSnapshot,
  NetworksetupServicesSnapshot,
  NetworksetupSnapshot,
  NetworksetupWifiSnapshot,
} from "../types";

// networksetup のスナップショット表示。サブコマンドごとに中身を切り替え、
// 続けて実行したコマンドの一覧を末尾に出す。状態・終了コード・stderr は OutputPane 側で出す。
export function NetworksetupResultView({ result }: { result: NetworksetupSnapshot }) {
  const { t } = useTranslation();

  return (
    <>
      {result.kind === "services" ? <ServicesView result={result} /> : null}
      {result.kind === "info" ? <InfoView result={result} /> : null}
      {result.kind === "wifi" ? <WifiView result={result} /> : null}
      {result.kind === "locations" ? <LocationsView result={result} /> : null}
      {result.commands.length > 1 ? (
        <details className="man-section">
          <summary>{t("networksetupResult.commands", { n: result.commands.length })}</summary>
          <ul className="extra-lines mono">
            {result.commands.map((command) => (
              <li key={command}>{command}</li>
            ))}
          </ul>
        </details>
      ) : null}
    </>
  );
}

function ServicesView({ result }: { result: NetworksetupServicesSnapshot }) {
  const { t } = useTranslation();
  if (result.services.length === 0) {
    return <p className="muted">{t("networksetupResult.noData")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th className="num">{t("networksetupResult.order")}</th>
            <th>{t("networksetupResult.service")}</th>
            <th>{t("networksetupResult.hardwarePort")}</th>
            <th>{t("networksetupResult.device")}</th>
            <th>{t("networksetupResult.enabled")}</th>
          </tr>
        </thead>
        <tbody>
          {result.services.map((service) => (
            <tr key={service.name}>
              <td className="num mono">{service.order ?? "—"}</td>
              <td>{service.name}</td>
              <td className="mono">{service.hardware_port || "—"}</td>
              <td className="mono">{service.device || "—"}</td>
              <td>{service.enabled ? t("networksetupResult.yes") : t("networksetupResult.no")}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function InfoView({ result }: { result: NetworksetupInfoSnapshot }) {
  const { t } = useTranslation();
  const none = t("networksetupResult.none");

  return (
    <>
      <div className="output-block">
        <h4>{t("networksetupResult.ip", { service: result.service })}</h4>
        {result.info.length === 0 ? (
          <p className="muted">{none}</p>
        ) : (
          <div className="table-wrap">
            <table className="top-table">
              <tbody>
                {result.info.map((entry) => (
                  <tr key={entry.key}>
                    <td>{entry.key}</td>
                    <td className="mono">{entry.value || "—"}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        )}
      </div>

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.dns")}</span>
          <span className="stat-value mono stat-small">
            {result.dns_servers.length > 0 ? result.dns_servers.join(", ") : t("networksetupResult.dnsAuto")}
          </span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.searchDomains")}</span>
          <span className="stat-value mono stat-small">
            {result.search_domains.length > 0 ? result.search_domains.join(", ") : none}
          </span>
        </div>
      </div>

      <div className="output-block">
        <h4>{t("networksetupResult.proxies")}</h4>
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("networksetupResult.proxyKind")}</th>
                <th>{t("networksetupResult.enabled")}</th>
                <th>{t("networksetupResult.server")}</th>
              </tr>
            </thead>
            <tbody>
              {result.proxies.map((proxy) => (
                <tr key={proxy.kind}>
                  <td>{t(`networksetupResult.proxy.${proxy.kind}`)}</td>
                  <td>{proxy.enabled ? t("networksetupResult.yes") : t("networksetupResult.no")}</td>
                  <td className="mono">
                    {proxy.server ? `${proxy.server}${proxy.port ? `:${proxy.port}` : ""}` : "—"}
                    {proxy.authenticated ? (
                      <span className="muted"> ({t("networksetupResult.authenticated")})</span>
                    ) : null}
                  </td>
                </tr>
              ))}
              <tr>
                <td>{t("networksetupResult.proxy.autoDiscovery")}</td>
                <td>
                  {result.auto_proxy_discovery ? t("networksetupResult.yes") : t("networksetupResult.no")}
                </td>
                <td>—</td>
              </tr>
              <tr>
                <td>{t("networksetupResult.proxy.pac")}</td>
                <td>{result.auto_proxy_enabled ? t("networksetupResult.yes") : t("networksetupResult.no")}</td>
                <td className="mono">{result.auto_proxy_url || "—"}</td>
              </tr>
            </tbody>
          </table>
        </div>
        {result.bypass_domains.length > 0 ? (
          <p className="note">
            {t("networksetupResult.bypass")}: <span className="mono">{result.bypass_domains.join(", ")}</span>
          </p>
        ) : null}
      </div>
    </>
  );
}

function WifiView({ result }: { result: NetworksetupWifiSnapshot }) {
  const { t } = useTranslation();
  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.device")}</span>
          <span className="stat-value mono">{result.device}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.power")}</span>
          <span className="stat-value">{result.power || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.network")}</span>
          <span className="stat-value stat-small">{result.network || "—"}</span>
        </div>
      </div>
      {!result.network && result.network_message ? (
        <p className="note">
          <span className="mono">{result.network_message}</span>
          <br />
          {t("networksetupResult.wifiHint")}
        </p>
      ) : null}
    </>
  );
}

function LocationsView({ result }: { result: NetworksetupLocationsSnapshot }) {
  const { t } = useTranslation();
  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("networksetupResult.currentLocation")}</span>
          <span className="stat-value">{result.current || "—"}</span>
        </div>
      </div>
      <ul className="extra-lines">
        {result.locations.map((location) => (
          <li key={location}>
            {location === result.current ? "● " : "○ "}
            {location}
          </li>
        ))}
      </ul>
    </>
  );
}
