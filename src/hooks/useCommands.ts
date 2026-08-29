import { useState, useEffect } from "react";
import { tauriService } from "../services/tauri";
import type { CommandSummary } from "../types";

export function useCommands() {
  const [commands, setCommands] = useState<CommandSummary[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadCommands();
  }, []);

  async function loadCommands() {
    try {
      setLoading(true);
      const data = await tauriService.getCommands();
      setCommands(data);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load commands");
    } finally {
      setLoading(false);
    }
  }

  return { commands, loading, error, refetch: loadCommands };
}
