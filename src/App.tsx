import { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { invoke } from "@tauri-apps/api/core";

interface DisablesleepState {
  enabled: boolean | null;
}

interface DisablesleepResult {
  success: boolean;
  exit_code: number;
  stdout: string;
  stderr: string;
  command: string;
}

function App() {
  const { t, i18n } = useTranslation();
  const [enabled, setEnabled] = useState(true);
  const [current, setCurrent] = useState<boolean | null>(null);
  const [remember, setRemember] = useState(true);
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<DisablesleepResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const preview = `sudo pmset -a disablesleep ${enabled ? 1 : 0}`;

  useEffect(() => {
    invoke<DisablesleepState>("get_disablesleep")
      .then((state) => {
        if (state.enabled !== null) {
          setCurrent(state.enabled);
          setEnabled(state.enabled);
        }
      })
      .catch(() => {
        // pmset -g が読めない環境では不明のままにする
      });
  }, []);

  async function handleExecute() {
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await invoke<DisablesleepResult>("set_disablesleep", {
        enabled,
      });
      setResult(res);
      if (res.success) setCurrent(enabled);
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  function toggleLang() {
    const next = i18n.language === "ja" ? "en" : "ja";
    i18n.changeLanguage(next);
    localStorage.setItem("manmen-lang", next);
  }

  async function handleRememberChange(next: boolean) {
    setRemember(next);
    await invoke("set_remember", { enabled: next });
  }

  return (
    <main className="container">
      <header className="header">
        <div>
          <h1>{t("app.title")}</h1>
          <p className="subtitle">{t("app.description")}</p>
        </div>
        <button className="btn-secondary" onClick={toggleLang}>
          {t("app.language")}
        </button>
      </header>

      <section className="card">
        <div className="row">
          <div>
            <h2>{t("sleep.title")}</h2>
            <p className="muted">
              {t("sleep.current")}:{" "}
              {current === null ? t("sleep.unknown") : current ? "ON (1)" : "OFF (0)"}
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
            onChange={(e) => handleRememberChange(e.target.checked)}
          />
          <span>{t("sleep.remember")}</span>
        </label>
        <p className="muted">{t("sleep.rememberNote")}</p>

        <button
          className="btn-primary"
          onClick={handleExecute}
          disabled={executing}
        >
          {executing ? t("sleep.executing") : t("sleep.execute")}
        </button>
      </section>

      {error && (
        <section className="card error">
          <h3>{t("result.failed")}</h3>
          <pre>{error}</pre>
        </section>
      )}
      {result && (
        <section className={`card ${result.success ? "success" : "error"}`}>
          <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>
          <p className="muted">
            {t("result.exitCode")}: {result.exit_code}
          </p>
          <div className="output-block">
            <h4>{t("result.output")}</h4>
            <pre>{result.stdout || t("result.noOutput")}</pre>
          </div>
          <div className="output-block">
            <h4>{t("result.error")}</h4>
            <pre>{result.stderr || t("result.noOutput")}</pre>
          </div>
        </section>
      )}

      <section className="card">
        <h2>{t("about.title")}</h2>
        <h3>{t("about.pmsetTitle")}</h3>
        <p className="muted">{t("about.pmset")}</p>
        <h3>{t("about.argsTitle")}</h3>
        <dl className="glossary">
          <dt>
            <code>sudo</code>
          </dt>
          <dd>{t("about.sudo")}</dd>
          <dt>
            <code>-a</code>
          </dt>
          <dd>{t("about.a")}</dd>
          <dt>
            <code>disablesleep</code>
          </dt>
          <dd>{t("about.disablesleep")}</dd>
        </dl>
        <h3>{t("about.onoffTitle")}</h3>
        <dl className="glossary">
          <dt>ON (1)</dt>
          <dd>{t("about.on")}</dd>
          <dt>OFF (0)</dt>
          <dd>{t("about.off")}</dd>
        </dl>
      </section>
    </main>
  );
}

export default App;
