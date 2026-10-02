import { useTranslation } from "react-i18next";
import {
  TOP_KEYS,
  TOP_MAX_COUNT,
  TOP_MAX_NCOLS,
  TOP_MIN_NCOLS,
  type TopCountMode,
  type TopKey,
  type TopSortOrder,
} from "../commands/topOptions";
import { useTop } from "../hooks/useTop";
import { TopResultView } from "./TopResultView";

const ORDERS: TopSortOrder[] = ["", "+", "-"];
const MODES: TopCountMode[] = ["n", "a", "d", "e"];

function Hint({ text }: { text: string }) {
  return <span className="field-hint">{text}</span>;
}

export function TopCard() {
  const { t } = useTranslation();
  const form = useTop();

  return (
    <>
      <section className="card">
        <h2>{t("top.title")}</h2>
        <p className="muted">{t("top.description")}</p>

        <div className="field-grid">
          <label className="field">
            <span className="field-label">{t("top.sort")}</span>
            <select
              value={form.sortKey}
              onChange={(e) => form.setSortKey(e.target.value as TopKey)}
            >
              {TOP_KEYS.map((key) => (
                <option key={key} value={key}>
                  {t(`topSort.${key}`)} (-o {key})
                </option>
              ))}
            </select>
            <Hint text={t("top.hintSort")} />
            <Hint text={t(`topHelp.${form.sortKey}`)} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.order")}</span>
            <select
              value={form.sortOrder}
              onChange={(e) => form.setSortOrder(e.target.value as TopSortOrder)}
            >
              {ORDERS.map((order) => (
                <option key={order || "default"} value={order}>
                  {t(`topOrder.${order || "default"}`)}
                </option>
              ))}
            </select>
            <Hint text={t(`topOrderHelp.${form.sortOrder || "default"}`)} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.secondary")}</span>
            <select
              value={form.secondaryKey}
              onChange={(e) => form.setSecondaryKey(e.target.value)}
            >
              <option value="">{t("top.none")}</option>
              {TOP_KEYS.map((key) => (
                <option key={key} value={key}>
                  {t(`topSort.${key}`)} (-O {key})
                </option>
              ))}
            </select>
            <Hint text={t("top.hintSecondary")} />
            {form.secondaryKey ? (
              <Hint text={t(`topHelp.${form.secondaryKey}`)} />
            ) : null}
          </label>
          <label className="field">
            <span className="field-label">{t("top.count")}</span>
            <input
              type="number"
              min={1}
              max={TOP_MAX_COUNT}
              value={form.count}
              onChange={(e) => form.setCount(Number(e.target.value))}
            />
            <Hint text={t("top.hintCount")} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.user")}</span>
            <input
              type="text"
              value={form.user}
              placeholder={t("top.userPlaceholder")}
              onChange={(e) => form.setUser(e.target.value)}
            />
            <Hint text={t("top.hintUser")} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.pids")}</span>
            <input
              type="text"
              value={form.pids}
              placeholder={t("top.pidsPlaceholder")}
              onChange={(e) => form.setPids(e.target.value)}
            />
            <Hint text={t("top.hintPids")} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.mode")}</span>
            <select
              value={form.countMode}
              onChange={(e) => form.setCountMode(e.target.value as TopCountMode)}
            >
              {MODES.map((mode) => (
                <option key={mode} value={mode}>
                  {t(`topMode.${mode}`)}
                </option>
              ))}
            </select>
            <Hint text={t(`topModeHelp.${form.countMode}`)} />
          </label>
          <label className="field">
            <span className="field-label">{t("top.ncols")}</span>
            <input
              type="number"
              min={TOP_MIN_NCOLS}
              max={TOP_MAX_NCOLS}
              value={form.ncols}
              placeholder={t("top.ncolsPlaceholder")}
              onChange={(e) => form.setNcols(e.target.value)}
            />
            <Hint text={t("top.hintNcols")} />
          </label>
        </div>

        <div className="choice-stack">
          <div className="flag-item">
            <label className="check-row">
              <input
                type="checkbox"
                checked={form.noFrameworks}
                onChange={(e) => form.setNoFrameworks(e.target.checked)}
              />
              <span>{t("topFlags.frameworks")}</span>
            </label>
            <Hint text={t("top.hintFrameworks")} />
          </div>
          <div className="flag-item">
            <label className="check-row">
              <input
                type="checkbox"
                checked={form.memoryMap}
                onChange={(e) => form.setMemoryMap(e.target.checked)}
              />
              <span>{t("topFlags.memoryMap")}</span>
            </label>
            <Hint text={t("top.hintMemoryMap")} />
          </div>
          <div className="flag-item">
            <label className="check-row">
              <input
                type="checkbox"
                checked={form.swap}
                onChange={(e) => form.setSwap(e.target.checked)}
              />
              <span>{t("topFlags.swap")}</span>
            </label>
            <Hint text={t("top.hintSwap")} />
          </div>
        </div>

        <details className="man-section">
          <summary>{t("top.stats")}</summary>
          <p className="muted">{t("top.statsHint")}</p>
          <div className="option-grid">
            {TOP_KEYS.map((key) => (
              <label key={key} className="option-item">
                <input
                  type="checkbox"
                  checked={form.stats.includes(key)}
                  onChange={() => form.toggleStat(key)}
                />
                <span className="option-copy">
                  <span className="option-title">
                    {t(`topSort.${key}`)} ({key})
                  </span>
                  <span className="field-hint">{t(`topHelp.${key}`)}</span>
                </span>
              </label>
            ))}
          </div>
        </details>

        {!form.userOk ? <p className="muted">{t("top.userInvalid")}</p> : null}
        {form.pidList === null ? <p className="muted">{t("top.pidsInvalid")}</p> : null}
        {!form.ncolsOk ? <p className="muted">{t("top.ncolsInvalid")}</p> : null}

        <button
          className="btn-primary"
          onClick={form.execute}
          disabled={form.executing || !form.valid}
        >
          {form.executing ? t("top.executing") : t("top.execute")}
        </button>

        <div className="preview">
          <span className="preview-label">{t("top.preview")}</span>
          <code>{form.preview}</code>
        </div>

        <p className="muted">{t("top.note")}</p>
      </section>

      <TopResultView result={form.result} error={form.error} />
    </>
  );
}
