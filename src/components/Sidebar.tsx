import { useTranslation } from "react-i18next";
import type { CommandEntry } from "../commands/registry";

export function Sidebar({
  entries,
  selectedId,
  onSelect,
}: {
  entries: CommandEntry[];
  selectedId: string;
  onSelect: (id: string) => void;
}) {
  const { t, i18n } = useTranslation();

  function toggleLang() {
    const next = i18n.language === "ja" ? "en" : "ja";
    i18n.changeLanguage(next);
    localStorage.setItem("manmen-lang", next);
  }

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <span className="brand-mark" aria-hidden>
          ❯_
        </span>
        <span className="brand-name">{t("app.title")}</span>
      </div>
      <nav>
        <ul>
          {entries.map((entry, index) => (
            <li key={entry.id}>
              <button
                type="button"
                className={entry.id === selectedId ? "is-selected" : ""}
                onClick={() => onSelect(entry.id)}
              >
                <code className="nav-cmd">{entry.cmd}</code>
                <span className="nav-label">{t(entry.nameKey)}</span>
                <kbd className="nav-key">⌘{index + 1}</kbd>
              </button>
            </li>
          ))}
        </ul>
      </nav>
      <div className="sidebar-foot">
        <button type="button" className="btn-ghost" onClick={toggleLang}>
          {t("app.language")}
        </button>
      </div>
    </aside>
  );
}
