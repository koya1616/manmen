import { useTranslation } from "react-i18next";
import {
  HIBERNATE_VALUES,
  MAX_PMSET_MINUTES,
  PMSET_SCOPES,
  PMSET_SETTINGS,
  type PmsetScope,
  type PmsetValueKind,
} from "../commands/pmsetSettings";
import { currentValue, usePmset } from "../hooks/usePmset";
import type { PmsetState } from "../types";
import { CommandResultView } from "./CommandResultView";

function formatScalar(
  kind: PmsetValueKind,
  value: string,
  t: (key: string) => string,
): string {
  if (kind === "bool") return value === "1" ? "ON (1)" : "OFF (0)";
  if (kind === "minutes") return `${value} ${t("pmset.minutesUnit")}`;
  return value;
}

function currentLabel(
  state: PmsetState | null,
  setting: string,
  scope: PmsetScope,
  kind: PmsetValueKind,
  t: (key: string) => string,
): string {
  if (setting === "disablesleep" || scope !== "a") {
    const value = currentValue(state, setting, scope === "a" ? "a" : scope);
    if (!value) return t("pmset.unknown");
    return formatScalar(kind, value, t);
  }
  const parts = (
    [
      ["b", state?.battery],
      ["c", state?.ac],
      ["u", state?.ups],
    ] as const
  )
    .map(([source, values]) => {
      const found = values?.find((item) => item.name === setting)?.value;
      if (!found) return null;
      return `${t(`pmsetScopeShort.${source}`)} ${formatScalar(kind, found, t)}`;
    })
    .filter((part): part is string => part !== null);
  return parts.length > 0 ? parts.join(" / ") : t("pmset.unknown");
}

export function PmsetCard() {
  const { t } = useTranslation();
  const {
    scope,
    setting,
    value,
    kind,
    state,
    remember,
    executing,
    result,
    error,
    valid,
    preview,
    chooseScope,
    chooseSetting,
    chooseValue,
    execute,
    updateRemember,
  } = usePmset();

  return (
    <>
      <section className="card">
        <h2>{t("pmset.title")}</h2>
        <p className="muted">{t("pmset.description")}</p>

        <div className="field">
          <span className="field-label">{t("pmset.scope")}</span>
          <div className="choice-row">
            {PMSET_SCOPES.map((item) => (
              <button
                key={item}
                type="button"
                className={scope === item ? "selected" : ""}
                onClick={() => chooseScope(item)}
              >
                {t(`pmsetScope.${item}`)}
              </button>
            ))}
          </div>
        </div>

        <label className="field">
          <span className="field-label">{t("pmset.setting")}</span>
          <select value={setting} onChange={(e) => chooseSetting(e.target.value)}>
            {PMSET_SETTINGS.map((item) => (
              <option key={item.name} value={item.name}>
                {item.name} — {t(`pmsetSettings.${item.name}`)}
              </option>
            ))}
          </select>
        </label>

        <p className="muted">
          {t("pmset.current")}: {currentLabel(state, setting, scope, kind, t)}
        </p>

        <div className="field">
          <span className="field-label">{t("pmset.value")}</span>
          {kind === "bool" ? (
            <button
              type="button"
              className={`switch ${value === "1" ? "on" : "off"}`}
              onClick={() => chooseValue(value === "1" ? "0" : "1")}
              aria-pressed={value === "1"}
            >
              {value === "1" ? "ON" : "OFF"}
            </button>
          ) : null}
          {kind === "minutes" ? (
            <div className="value-row">
              <input
                type="number"
                min={0}
                max={MAX_PMSET_MINUTES}
                className="search-input value-input"
                value={value}
                onChange={(e) => chooseValue(e.target.value)}
                aria-label={t("pmset.value")}
              />
              <span className="muted">{t("pmset.minutesHint")}</span>
            </div>
          ) : null}
          {kind === "hibernate" ? (
            <div className="choice-row">
              {HIBERNATE_VALUES.map((item) => (
                <button
                  key={item}
                  type="button"
                  className={value === item ? "selected" : ""}
                  onClick={() => chooseValue(item)}
                >
                  {item} — {t(`pmsetHibernate.${item}`)}
                </button>
              ))}
            </div>
          ) : null}
        </div>

        <div className="preview">
          <span className="preview-label">{t("pmset.preview")}</span>
          <code>{preview}</code>
        </div>

        <p className="muted">{t("pmset.adminNote")}</p>

        <label className="check-row">
          <input
            type="checkbox"
            checked={remember}
            onChange={(e) => updateRemember(e.target.checked)}
          />
          <span>{t("pmset.remember")}</span>
        </label>
        <p className="muted">{t("pmset.rememberNote")}</p>

        <button
          className="btn-primary"
          onClick={execute}
          disabled={executing || !valid}
        >
          {executing ? t("pmset.executing") : t("pmset.execute")}
        </button>
      </section>

      <CommandResultView result={result} error={error} />
    </>
  );
}
