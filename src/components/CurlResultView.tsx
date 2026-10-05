import { useTranslation } from "react-i18next";
import type { CurlInfo, CurlResponse, CurlSnapshot } from "../types";

// curl のスナップショット表示。概要 + 所要時間 + レスポンスヘッダ + 本文で見せる。
// 状態・終了コード・stderr は OutputPane 側で出す。
export function CurlResultView({ result }: { result: CurlSnapshot }) {
  const { t } = useTranslation();
  const { info, responses } = result;
  const final = responses.length > 0 ? responses[responses.length - 1] : null;
  const earlier = responses.slice(0, -1);

  return (
    <>
      {info ? <CurlSummary info={info} /> : null}
      {info && info.time_total > 0 ? <CurlTiming info={info} /> : null}

      {earlier.length > 0 ? (
        <div className="output-block">
          <h4>{t("curlResult.redirects")}</h4>
          {earlier.map((response, index) => (
            <details key={index} className="man-section">
              <summary className="mono">{response.status_line}</summary>
              <HeaderTable response={response} />
            </details>
          ))}
        </div>
      ) : null}

      {final ? (
        <div className="output-block">
          <h4>
            {t("curlResult.headers")} <span className="muted mono">{final.status_line}</span>
          </h4>
          <HeaderTable response={final} />
        </div>
      ) : null}

      <CurlBody result={result} />
    </>
  );
}

function CurlSummary({ info }: { info: CurlInfo }) {
  const { t } = useTranslation();
  const remote =
    info.remote_ip && info.remote_ip !== ""
      ? `${info.remote_ip}${info.remote_port ? `:${info.remote_port}` : ""}`
      : null;

  return (
    <div className="stat-grid">
      <div className="stat">
        <span className="stat-label">{t("curlResult.status")}</span>
        <span className="stat-value mono">{info.http_code || "—"}</span>
      </div>
      <div className="stat">
        <span className="stat-label">{t("curlResult.total")}</span>
        <span className="stat-value mono">{formatMs(info.time_total)}</span>
      </div>
      <div className="stat">
        <span className="stat-label">{t("curlResult.size")}</span>
        <span className="stat-value mono">{formatBytes(info.size_download)}</span>
      </div>
      {info.http_version && info.http_version !== "0" ? (
        <div className="stat">
          <span className="stat-label">{t("curlResult.httpVersion")}</span>
          <span className="stat-value mono">HTTP/{info.http_version}</span>
        </div>
      ) : null}
      {remote ? (
        <div className="stat">
          <span className="stat-label">{t("curlResult.remote")}</span>
          <span className="stat-value mono stat-small">{remote}</span>
        </div>
      ) : null}
      {info.num_redirects > 0 ? (
        <div className="stat">
          <span className="stat-label">{t("curlResult.numRedirects")}</span>
          <span className="stat-value">{info.num_redirects}</span>
        </div>
      ) : null}
      {info.content_type ? (
        <div className="stat">
          <span className="stat-label">{t("curlResult.contentType")}</span>
          <span className="stat-value mono stat-small">{info.content_type}</span>
        </div>
      ) : null}
      {info.url_effective && info.num_redirects > 0 ? (
        <div className="stat">
          <span className="stat-label">{t("curlResult.finalUrl")}</span>
          <span className="stat-value mono stat-small">{info.url_effective}</span>
        </div>
      ) : null}
    </div>
  );
}

// 各値は curl の開始からの累積秒。合計に対する割合でバーを描く。
function CurlTiming({ info }: { info: CurlInfo }) {
  const { t } = useTranslation();
  const rows: { key: string; value: number }[] = [
    { key: "namelookup", value: info.time_namelookup },
    { key: "connect", value: info.time_connect },
    { key: "appconnect", value: info.time_appconnect },
    { key: "redirect", value: info.time_redirect },
    { key: "starttransfer", value: info.time_starttransfer },
    { key: "total", value: info.time_total },
  ].filter((row) => row.key === "total" || row.value > 0);

  return (
    <div className="output-block">
      <h4>{t("curlResult.timing")}</h4>
      <div className="table-wrap">
        <table className="top-table">
          <tbody>
            {rows.map((row) => (
              <tr key={row.key}>
                <td>{t(`curlResult.time.${row.key}`)}</td>
                <td className="num mono">{formatMs(row.value)}</td>
                <td className="curl-timing-cell">
                  <div
                    className="curl-timing-bar"
                    style={{ width: `${Math.min(100, (row.value / info.time_total) * 100)}%` }}
                  />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}

function HeaderTable({ response }: { response: CurlResponse }) {
  const { t } = useTranslation();
  if (response.headers.length === 0) {
    return <p className="muted">{t("curlResult.noHeaders")}</p>;
  }
  return (
    <div className="table-wrap">
      <table className="top-table">
        <tbody>
          {response.headers.map((header, index) => (
            <tr key={index}>
              <td className="mono">{header.name}</td>
              <td className="mono curl-header-value">{header.value}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function CurlBody({ result }: { result: CurlSnapshot }) {
  const { t } = useTranslation();
  if (result.body_binary) {
    return <p className="muted">{t("curlResult.binary")}</p>;
  }
  if (result.body === "") {
    return result.responses.length > 0 ? (
      <p className="muted">{t("curlResult.noBody")}</p>
    ) : null;
  }
  return (
    <div className="output-block">
      <h4>{t("curlResult.body")}</h4>
      <pre className="term">{prettyBody(result.body, result.info?.content_type ?? null)}</pre>
      {result.body_truncated ? <p className="note">{t("curlResult.truncated")}</p> : null}
    </div>
  );
}

function prettyBody(body: string, contentType: string | null): string {
  if (contentType && /json/i.test(contentType)) {
    try {
      return JSON.stringify(JSON.parse(body), null, 2);
    } catch {
      return body;
    }
  }
  return body;
}

function formatMs(seconds: number): string {
  return `${(seconds * 1000).toFixed(1)} ms`;
}

function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / 1024 / 1024).toFixed(1)} MB`;
}
