import { useState } from "react";
import { api } from "../api";
import type { ManpageDocument } from "../types";
import { DEFAULT_MAN_TOPIC } from "../commands/manTopics";

export function useManpage() {
  const [topic, setTopic] = useState(DEFAULT_MAN_TOPIC);
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<ManpageDocument | null>(null);
  const [error, setError] = useState<string | null>(null);

  const preview = `man ${topic.trim() || "..."}`;

  async function execute() {
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await api.getManpage(topic);
      setResult(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  return { topic, setTopic, executing, result, error, preview, execute };
}
