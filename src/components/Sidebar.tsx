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
  const { t } = useTranslation();

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>{t("app.title")}</h1>
      </div>
      <nav>
        <ul>
          {entries.map((entry) => (
            <li key={entry.id}>
              <button
                className={entry.id === selectedId ? "selected" : ""}
                onClick={() => onSelect(entry.id)}
              >
                {t(entry.nameKey)}
              </button>
            </li>
          ))}
        </ul>
      </nav>
    </aside>
  );
}
