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

  return (
    <aside className="sidebar">
      <div className="sidebar-header">
        <h1>Manmen</h1>
        <p className="subtitle">macOS CLI GUI Manager</p>
      </div>

      <div className="search-box">
        <input
          type="text"
          placeholder="Search commands..."
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
        />
      </div>

      <nav className="command-list">
        {loading ? (
          <p className="loading">Loading commands...</p>
        ) : commands.length === 0 ? (
          <p className="no-commands">No commands found</p>
        ) : (
          Object.entries(grouped).map(([category, cmds]) => (
            <div key={category} className="category-group">
              <h3 className="category-name">{category}</h3>
              <ul>
                {cmds.map((cmd) => (
                  <li
                    key={cmd.id}
                    className={`command-item ${selectedId === cmd.id ? "selected" : ""}`}
                    onClick={() => onSelect(cmd.id)}
                  >
                    <span className="command-name">{cmd.name}</span>
                    {cmd.requires_admin && <span className="badge admin">A</span>}
                    {cmd.dangerous && <span className="badge dangerous">!</span>}
                  </li>
                ))}
              </ul>
            </div>
          ))
        )}
      </nav>
    </aside>
  );
}
