import { useTranslation } from "react-i18next";
import type { NamesSnapshot } from "../types";

// scutil --get 3種の表示。未設定は空欄ではなく文言で示す。
export function ScutilNamesResultView({ result }: { result: NamesSnapshot }) {
  const { t } = useTranslation();

  return (
    <div className="table-wrap">
      <table className="top-table">
        <thead>
          <tr>
            <th>{t("scutilNamesResult.key")}</th>
            <th>{t("scutilNamesResult.value")}</th>
          </tr>
        </thead>
        <tbody>
          {result.names.map((entry) => (
            <tr key={entry.key}>
              <td className="mono">{entry.key}</td>
              <td className="mono">
                {entry.value ? entry.value : <span className="muted">{t("scutilNamesResult.unset")}</span>}
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
