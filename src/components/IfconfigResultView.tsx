import { useTranslation } from "react-i18next";
import type { IfconfigInterface, IfconfigSnapshot } from "../types";

// ifconfig のスナップショット表示。主な IPv4 の概要 + インターフェース表で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function IfconfigResultView({
  result,
  activeOnly,
}: {
  result: IfconfigSnapshot;
  activeOnly: boolean;
}) {
  const { t } = useTranslation();
  const interfaces = activeOnly ? result.interfaces.filter(isInUse) : result.interfaces;
  const hidden = result.interfaces.length - interfaces.length;
  const primary = result.interfaces.find(
    (iface) => iface.status === "active" && iface.inet.length > 0 && !iface.flags.includes("LOOPBACK"),
  );

  return (
    <>
      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("ifconfigResult.primary")}</span>
          <span className="stat-value mono">{primary ? primary.inet[0].address : "—"}</span>
        </div>
        {primary ? (
          <div className="stat">
            <span className="stat-label">{t("ifconfigResult.primaryIface")}</span>
            <span className="stat-value mono stat-small">
              {primary.name}
              {primary.hardware_port ? ` (${primary.hardware_port})` : ""}
            </span>
          </div>
        ) : null}
        <div className="stat">
          <span className="stat-label">{t("ifconfigResult.count")}</span>
          <span className="stat-value">{result.interfaces.length}</span>
        </div>
      </div>

      {interfaces.length === 0 ? (
        <p className="muted">{t("ifconfigResult.noData")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <thead>
              <tr>
                <th>{t("ifconfigResult.interface")}</th>
                <th>{t("ifconfigResult.status")}</th>
                <th>IPv4</th>
                <th>IPv6</th>
                <th>MAC</th>
                <th className="num">MTU</th>
              </tr>
            </thead>
            <tbody>
              {interfaces.map((iface) => (
                <tr key={iface.name}>
                  <td>
                    <div className="mono">{iface.name}</div>
                    <div className="muted">{kindLabel(iface, t)}</div>
                  </td>
                  <td>{statusLabel(iface, t)}</td>
                  <td className="mono">
                    {iface.inet.length === 0
                      ? "—"
                      : iface.inet.map((inet) => (
                          <div key={inet.address}>
                            {inet.address}
                            {inet.prefix_len !== null ? `/${inet.prefix_len}` : ""}
                            {inet.destination && inet.destination !== inet.address ? (
                              <span className="muted"> → {inet.destination}</span>
                            ) : null}
                          </div>
                        ))}
                  </td>
                  <td className="mono">
                    {iface.inet6.length === 0
                      ? "—"
                      : iface.inet6.map((inet6) => (
                          <div key={inet6.address} className={inet6.link_local ? "muted" : ""}>
                            {inet6.address}
                            {inet6.prefix_len !== null ? `/${inet6.prefix_len}` : ""}
                            {inet6.attributes.includes("temporary") ? (
                              <span className="muted"> ({t("ifconfigResult.temporary")})</span>
                            ) : null}
                          </div>
                        ))}
                  </td>
                  <td className="mono">{iface.mac || "—"}</td>
                  <td className="num mono">{iface.mtu ?? "—"}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
      {hidden > 0 ? <p className="note">{t("ifconfigResult.hidden", { n: hidden })}</p> : null}
      <p className="note">{t("ifconfigResult.hint")}</p>
    </>
  );
}

// 「使用中」= リンクが active か、リンクローカル以外のアドレスを持つもの。
function isInUse(iface: IfconfigInterface): boolean {
  return (
    iface.status === "active" ||
    iface.inet.length > 0 ||
    iface.inet6.some((inet6) => !inet6.link_local)
  );
}

function statusLabel(iface: IfconfigInterface, t: (key: string) => string): string {
  if (iface.status === "active") return t("ifconfigResult.active");
  if (iface.status === "inactive") return t("ifconfigResult.inactive");
  return iface.flags.includes("UP") ? "UP" : t("ifconfigResult.down");
}

// networksetup のハードウェアポート名が無ければ、名前の接頭辞から用途を推定する。
const KIND_PREFIXES: [string, string][] = [
  ["lo", "loopback"],
  ["utun", "tunnel"],
  ["ipsec", "tunnel"],
  ["awdl", "awdl"],
  ["llw", "llw"],
  ["bridge", "bridge"],
  ["ap", "ap"],
  ["anpi", "internal"],
  ["gif", "ipv6tunnel"],
  ["stf", "ipv6tunnel"],
];

function kindLabel(iface: IfconfigInterface, t: (key: string) => string): string {
  if (iface.hardware_port) return iface.hardware_port;
  const prefix = iface.name.replace(/[0-9]+$/, "");
  const kind = KIND_PREFIXES.find(([p]) => p === prefix)?.[1];
  return kind ? t(`ifconfigKind.${kind}`) : "";
}
