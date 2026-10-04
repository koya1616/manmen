import { useTranslation } from "react-i18next";
import type { NwiSnapshot } from "../types";

// scutil --nwi の表示。セクションごとに iface/key/value の表を並べる。
export function ScutilNwiResultView({ result }: { result: NwiSnapshot }) {
  const { t } = useTranslation();

  if (result.sections.length === 0) {
    return <p className="muted">{t("scutilNwiResult.noData")}</p>;
  }
  return (
    <>
      {result.sections.map((section) => (
        <section key={section.title}>
          <h4>{section.title}</h4>
          <div className="table-wrap">
            <table className="top-table">
              <thead>
                <tr>
                  <th>{t("scutilNwiResult.iface")}</th>
                  <th>{t("scutilNwiResult.key")}</th>
                  <th>{t("scutilNwiResult.value")}</th>
                </tr>
              </thead>
              <tbody>
                {section.rows.map((row, index) => (
                  <tr key={`${row.iface}-${row.key}-${index}`}>
                    <td className="mono">{row.iface}</td>
                    <td className="mono">{row.key}</td>
                    <td className="mono">{row.value}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </section>
      ))}
      {result.footer ? (
        <p className="note mono">
          {t("scutilNwiResult.interfaces")}: {result.footer}
        </p>
      ) : null}
    </>
  );
}
