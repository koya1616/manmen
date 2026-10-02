import { useTranslation } from "react-i18next";
import type { GcPolicy, InspectSnapshot } from "../types";

function GcPolicyTable({ policy }: { policy: GcPolicy }) {
  const { t } = useTranslation();
  const rows: [string, string][] = [
    [t("dockerInspect.gcAll"), policy.all],
    [t("dockerInspect.gcFilters"), policy.filters],
    [t("dockerInspect.gcKeep"), policy.keep_duration],
    [t("dockerInspect.gcMax"), policy.max_used_space],
    [t("dockerInspect.gcReserved"), policy.reserved_space],
    [t("dockerInspect.gcMinFree"), policy.min_free_space],
  ];
  const visible = rows.filter(([, value]) => value !== "");

  return (
    <div className="output-block">
      <h4>
        <code>GC Policy {policy.name}</code>
      </h4>
      {visible.length === 0 && policy.extra.length === 0 ? (
        <p className="muted">{t("dockerInspect.gcEmpty")}</p>
      ) : (
        <div className="table-wrap">
          <table className="top-table">
            <tbody>
              {visible.map(([label, value]) => (
                <tr key={label}>
                  <td>{label}</td>
                  <td className="mono">{value}</td>
                </tr>
              ))}
              {policy.extra.map((line) => (
                <tr key={line}>
                  <td>—</td>
                  <td className="mono">{line}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
}

// `docker builder inspect` の構造化表示。先頭部 + ノードごとの詳細で見せる。
export function DockerInspectResultView({
  result,
  error,
}: {
  result: InspectSnapshot | null;
  error: string | null;
}) {
  const { t } = useTranslation();

  if (error) {
    return (
      <section className="card error">
        <h3>{t("result.failed")}</h3>
        <pre>{error}</pre>
      </section>
    );
  }

  if (!result) return null;

  return (
    <section className={`card ${result.success ? "success" : "error"}`}>
      <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>

      <div className="stat-grid">
        <div className="stat">
          <span className="stat-label">{t("dockerInspect.name")}</span>
          <span className="stat-value">{result.name || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerInspect.driver")}</span>
          <span className="stat-value">{result.driver || "—"}</span>
        </div>
        <div className="stat">
          <span className="stat-label">{t("dockerInspect.lastActivity")}</span>
          <span className="stat-value stat-small">{result.last_activity || "—"}</span>
        </div>
      </div>

      {result.nodes.length === 0 ? (
        <p className="muted">{t("dockerInspect.noData")}</p>
      ) : (
        result.nodes.map((node) => (
          <div key={node.name || "node"} className="output-block">
            <h4>
              <code>{node.name || "—"}</code>
              {node.status ? <span className="muted"> — {node.status}</span> : null}
            </h4>
            <div className="table-wrap">
              <table className="top-table">
                <tbody>
                  <tr>
                    <td>{t("dockerInspect.endpoint")}</td>
                    <td className="mono">{node.endpoint || "—"}</td>
                  </tr>
                  <tr>
                    <td>{t("dockerInspect.buildkit")}</td>
                    <td className="mono">{node.buildkit || "—"}</td>
                  </tr>
                  <tr>
                    <td>{t("dockerInspect.platforms")}</td>
                    <td className="mono">{node.platforms || "—"}</td>
                  </tr>
                </tbody>
              </table>
            </div>

            {node.labels.length > 0 ? (
              <div className="output-block">
                <h4>{t("dockerInspect.labels")}</h4>
                <ul className="extra-lines">
                  {node.labels.map((label) => (
                    <li key={label} className="mono">
                      {label}
                    </li>
                  ))}
                </ul>
              </div>
            ) : null}

            {node.devices.length > 0 ? (
              <div className="output-block">
                <h4>{t("dockerInspect.devices")}</h4>
                <ul className="extra-lines">
                  {node.devices.map((device) => (
                    <li key={device} className="mono">
                      {device}
                    </li>
                  ))}
                </ul>
              </div>
            ) : null}

            {node.gc_policies.map((policy) => (
              <GcPolicyTable key={policy.name || "gc"} policy={policy} />
            ))}
          </div>
        ))
      )}

      {result.stderr ? (
        <div className="output-block">
          <h4>{t("result.error")}</h4>
          <pre>{result.stderr}</pre>
        </div>
      ) : null}

      <details className="raw-details">
        <summary>{t("dockerInspect.raw")}</summary>
        <p className="muted">
          {t("result.exitCode")}: {result.exit_code}
        </p>
        <p className="muted">
          <code>{result.command}</code>
        </p>
      </details>
    </section>
  );
}
