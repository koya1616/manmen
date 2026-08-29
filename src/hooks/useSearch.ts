import { useState } from "react";
import { tauriService } from "../services/tauri";
import type { CommandSummary } from "../types";

export function useSearch() {
  const [query, setQuery] = useState("");
  const [results, setResults] = useState<CommandSummary[]>([]);
  const [loading, setLoading] = useState(false);

  async function search(searchQuery: string) {
    setQuery(searchQuery);

    if (!searchQuery.trim()) {
      setResults([]);
      return;
    }

    try {
      setLoading(true);
      const data = await tauriService.searchCommands(searchQuery);
      setResults(data);
    } catch (err) {
      console.error("Search failed:", err);
      setResults([]);
    } finally {
      setLoading(false);
    }
  }

  return { query, results, loading, search, setQuery };
}
