import { useTranslation } from "react-i18next";
import type { CommandSummary } from "../../types";

interface SidebarProps {
  commands: CommandSummary[];
  selectedId: string | null;
  onSelect: (id: string) => void;
  searchQuery: string;
  onSearchChange: (query: string) => void;
  loading: boolean;
}

export function Sidebar({
  commands,
  selectedId,
  onSelect,
  searchQuery,
  onSearchChange,
  loading,
}: SidebarProps) {
  const { t, i18n } = useTranslation();

  const grouped = commands.reduce(
    (acc, cmd) => {
      if (!acc[cmd.category]) {
        acc[cmd.category] = [];
      }
      acc[cmd.category].push(cmd);
      return acc;
    },
    {} as Record<string, CommandSummary[]>
  );

  const toggleLanguage = () => {
    const newLang = i18n.language.startsWith("ja") ? "en" : "ja";
    i18n.changeLanguage(newLang);
    localStorage.setItem("manmen-lang", newLang);
  };

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>{t("app.name")}</h1>
        <p className="subtitle">{t("app.subtitle")}</p>
      </div>

      <div className="search-box">
        <input
          type="text"
          placeholder={t("sidebar.searchPlaceholder")}
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
        />
      </div>

      <nav className="command-list">
        {loading ? (
          <p className="loading">{t("sidebar.loading")}</p>
        ) : commands.length === 0 ? (
          <p className="no-commands">{t("sidebar.noCommands")}</p>
        ) : (
          Object.entries(grouped).map(([category, cmds]) => (
            <div key={category} className="category-group">
              <h3 className="category-name">{t(`categories.${category}`, category)}</h3>
              <ul>
                {cmds.map((cmd) => (
                  <li
                    key={cmd.id}
                    className={`command-item ${selectedId === cmd.id ? "selected" : ""}`}
                    onClick={() => onSelect(cmd.id)}
                  >
                    <span className="command-name">{t(`commands.${cmd.id}.name`, cmd.name)}</span>
                    {cmd.requires_admin && <span className="badge admin">A</span>}
                    {cmd.dangerous && <span className="badge dangerous">!</span>}
                  </li>
                ))}
              </ul>
            </div>
          ))
        )}
      </nav>

      <div className="language-toggle">
        <button onClick={toggleLanguage} className="btn-language">
          {i18n.language.startsWith("ja") ? "EN" : "JA"}
        </button>
      </div>
    </aside>
  );
}
