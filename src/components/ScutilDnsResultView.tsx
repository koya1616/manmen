import { useTranslation } from "react-i18next";
import type { DnsSnapshot } from "../types";

// scutil --dns の表示。セクションごとに resolver の表を並べる。
export function ScutilDnsResultView({ result }: { result: DnsSnapshot }) {
  const { t } = useTranslation();

  if (result.sections.length === 0) {
    return <p className="muted">{t("scutilDnsResult.noData")}</p>;
  }
  return (
    <>
      {result.sections.map((section) => (
        <section key={section.title}>
          <h4>{section.title}</h4>
          {section.resolvers.map((resolver) => (
            <div key={resolver.name}>
              <p className="mono muted">{resolver.name}</p>
              <div className="table-wrap">
                <table className="top-table">
                  <thead>
                    <tr>
                      <th>{t("scutilDnsResult.key")}</th>
                      <th>{t("scutilDnsResult.value")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {resolver.entries.map((entry, index) => (
                      <tr key={`${entry.key}-${index}`}>
                        <td className="mono">{entry.key}</td>
                        <td className="mono">{entry.value}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          ))}
        </section>
      ))}
    </>
  );
}
