import { useTranslation } from "react-i18next";
import { TOP_MAX_COUNT, TOP_SORT_KEYS, useTop } from "../hooks/useTop";
import { TopResultView } from "./TopResultView";

export function TopCard() {
  const { t } = useTranslation();
  const {
    sortKey,
    setSortKey,
    count,
    setCount,
    executing,
    result,
    error,
    preview,
    isValidCount,
    execute,
  } = useTop();

  return (
    <>
      <section className="card">
        <h2>{t("top.title")}</h2>
        <p className="muted">{t("top.description")}</p>

        <div className="topic-row">
          <label className="muted" htmlFor="top-sort">
            {t("top.sort")}
          </label>
          <select
            id="top-sort"
            value={sortKey}
            onChange={(e) =>
              setSortKey(e.target.value as typeof sortKey)
            }
          >
            {TOP_SORT_KEYS.map((key) => (
              <option key={key} value={key}>
                {t(`topSort.${key}`)} (-o {key})
              </option>
            ))}
          </select>
          <label className="muted" htmlFor="top-count">
            {t("top.count")}
          </label>
          <input
            id="top-count"
            type="number"
            min={1}
            max={TOP_MAX_COUNT}
            value={count}
            onChange={(e) => setCount(Number(e.target.value))}
          />
        </div>

        <div className="topic-row">
          <button
            className="btn-primary"
            onClick={execute}
            disabled={executing || !isValidCount}
          >
            {executing ? t("top.executing") : t("top.execute")}
          </button>
        </div>

        <div className="preview">
          <span className="preview-label">{t("top.preview")}</span>
          <code>{preview}</code>
        </div>

        <p className="muted">{t("top.note")}</p>
      </section>

      <TopResultView result={result} error={error} />
    </>
  );
}
