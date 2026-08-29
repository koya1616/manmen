import { useState, useEffect } from "react";
import { Sidebar } from "./components/layout/Sidebar";
import { CommandDetailView } from "./components/command/CommandDetailView";
import { ExecutionResult } from "./components/result/ExecutionResult";
import { useCommands } from "./hooks/useCommands";
import { useCommandExecution } from "./hooks/useCommandExecution";
import { tauriService } from "./services/tauri";
import type { HistoryEntry } from "./types";

type View = "home" | "command" | "history";

function App() {
  const { commands, loading: commandsLoading } = useCommands();
  const {
    command,
    args,
    validation,
    preview,
    result,
    executing,
    error,
    loadCommand,
    updateArg,
    validate,
    build,
    execute,
  } = useCommandExecution();

  const [view, setView] = useState<View>("home");
  const [searchQuery, setSearchQuery] = useState("");
  const [history, setHistory] = useState<HistoryEntry[]>([]);
  const [historyLoading, setHistoryLoading] = useState(false);

  useEffect(() => {
    if (view === "history") {
      loadHistory();
    }
  }, [view]);

  async function loadHistory() {
    try {
      setHistoryLoading(true);
      const data = await tauriService.getExecutionHistory(50);
      setHistory(data);
    } catch (err) {
      console.error("Failed to load history:", err);
    } finally {
      setHistoryLoading(false);
    }
  }

  async function handleSelectCommand(id: string) {
    await loadCommand(id);
    setView("command");
  }

  async function handleExecute() {
    await validate();
    const v = validation;
    if (v && !v.valid) return;

    await execute();
  }

  return (
    <div className="app">
      <Sidebar
        commands={commands}
        selectedId={command?.id || null}
        onSelect={handleSelectCommand}
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        loading={commandsLoading}
      />

      <main className="main-content">
        {view === "home" && (
          <div className="home-view">
            <h2>Welcome to Manmen</h2>
            <p>macOS CLI GUI Manager</p>
            <div className="quick-actions">
              <button onClick={() => setView("history")}>
                View Execution History
              </button>
            </div>
          </div>
        )}

        {view === "command" && command && (
          <>
            <CommandDetailView
              command={command}
              args={args}
              validation={validation}
              preview={preview}
              executing={executing}
              error={error}
              onArgChange={updateArg}
              onValidate={validate}
              onBuild={build}
              onExecute={handleExecute}
            />
            {result && <ExecutionResult result={result} />}
          </>
        )}

        {view === "history" && (
          <div className="history-view">
            <h2>Execution History</h2>
            {historyLoading ? (
              <p>Loading history...</p>
            ) : history.length === 0 ? (
              <p>No execution history yet.</p>
            ) : (
              <ul className="history-list">
                {history.map((entry) => (
                  <li key={entry.id} className="history-item">
                    <span className={`status ${entry.success ? "success" : "failure"}`}>
                      {entry.success ? "✓" : "✕"}
                    </span>
                    <code>{entry.generated_command}</code>
                    <span className="time">
                      {new Date(entry.started_at).toLocaleString()}
                    </span>
                  </li>
                ))}
              </ul>
            )}
          </div>
        )}
      </main>
    </div>
  );
}

export default App;
