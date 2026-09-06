import { useState } from "react";
import { useTranslation } from "react-i18next";
import { commandRegistry } from "./commands/registry";
import { Sidebar } from "./components/Sidebar";

function App() {
  const { t, i18n } = useTranslation();
  const [selectedId, setSelectedId] = useState(commandRegistry[0].id);

  const selected =
    commandRegistry.find((entry) => entry.id === selectedId) ??
    commandRegistry[0];
  const Control = selected.control;
  const About = selected.about;

  function toggleLang() {
    const next = i18n.language === "ja" ? "en" : "ja";
    i18n.changeLanguage(next);
    localStorage.setItem("manmen-lang", next);
  }

  return (
    <div className="app">
      <Sidebar
        entries={commandRegistry}
        selectedId={selected.id}
        onSelect={setSelectedId}
      />
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

        <Control />
        <About />
      </main>
    </div>
  );
}

export default App;
