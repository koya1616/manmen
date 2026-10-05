import { useEffect, useState } from "react";
import { commandRegistry } from "./commands/registry";
import { CommandPalette } from "./components/CommandPalette";
import { Sidebar } from "./components/Sidebar";

function App() {
  const [selectedId, setSelectedId] = useState(commandRegistry[0].id);
  const [paletteOpen, setPaletteOpen] = useState(false);

  // ⌘K で検索を開閉し、⌘1〜⌘9 でコマンドを切り替える
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if (!e.metaKey || e.shiftKey || e.altKey) return;
      if (e.key.toLowerCase() === "k") {
        e.preventDefault();
        setPaletteOpen((open) => !open);
        return;
      }
      const index = Number(e.key) - 1;
      const entry = commandRegistry[index];
      if (entry) {
        e.preventDefault();
        setSelectedId(entry.id);
      }
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, []);

  // 切り替えても入力と出力が残るよう、全コマンドをマウントしたまま表示だけ切り替える
  return (
    <div className="app">
      <Sidebar
        entries={commandRegistry}
        selectedId={selectedId}
        onSelect={setSelectedId}
        onOpenSearch={() => setPaletteOpen(true)}
      />
      <main className="main">
        {commandRegistry.map((entry) => {
          const Control = entry.control;
          const About = entry.about;
          return <Control key={entry.id} active={entry.id === selectedId} about={<About />} />;
        })}
      </main>
      <CommandPalette
        entries={commandRegistry}
        open={paletteOpen}
        onClose={() => setPaletteOpen(false)}
        onSelect={setSelectedId}
      />
    </div>
  );
}

export default App;
