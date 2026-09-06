import { useTranslation } from "react-i18next";
import { commandRegistry } from "./commands/registry";

function App() {
  const { t, i18n } = useTranslation();

  function toggleLang() {
    const next = i18n.language === "ja" ? "en" : "ja";
    i18n.changeLanguage(next);
    localStorage.setItem("manmen-lang", next);
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

      {commandRegistry.map((entry) => (
        <entry.control key={`${entry.id}-control`} />
      ))}
      {commandRegistry.map((entry) => (
        <entry.about key={`${entry.id}-about`} />
      ))}
    </main>
  );
}

export default App;
