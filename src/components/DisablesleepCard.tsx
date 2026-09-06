import { useTranslation } from "react-i18next";
import { useDisablesleep } from "../hooks/useDisablesleep";
import { CommandResultView } from "./CommandResultView";

export function DisablesleepCard() {
  const { t } = useTranslation();
  const {
    enabled,
    setEnabled,
    current,
    remember,
    executing,
    result,
    error,
    preview,
    execute,
    updateRemember,
  } = useDisablesleep();

  return (
    <>
      <section className="card">
        <div className="row">
          <div>
            <h2>{t("sleep.title")}</h2>
            <p className="muted">
              {t("sleep.current")}:{" "}
              {current === null
                ? t("sleep.unknown")
                : current
                  ? "ON (1)"
                  : "OFF (0)"}
            </p>
          </div>
          <button
            className={`switch ${enabled ? "on" : "off"}`}
            onClick={() => setEnabled((v) => !v)}
            aria-pressed={enabled}
          >
            {enabled ? "ON" : "OFF"}
          </button>
        </div>

        <div className="preview">
          <span className="preview-label">{t("sleep.preview")}</span>
          <code>{preview}</code>
        </div>

        <p className="muted">{t("sleep.adminNote")}</p>

        <label className="check-row">
          <input
            type="checkbox"
            checked={remember}
            onChange={(e) => updateRemember(e.target.checked)}
          />
          <span>{t("sleep.remember")}</span>
        </label>
        <p className="muted">{t("sleep.rememberNote")}</p>

        <button
          className="btn-primary"
          onClick={execute}
          disabled={executing}
        >
          {executing ? t("sleep.executing") : t("sleep.execute")}
        </button>
      </section>

      <CommandResultView result={result} error={error} />
    </>
  );
}
