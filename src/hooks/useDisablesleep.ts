import { useEffect, useState } from "react";
import { api } from "../api";
import type { CommandResult } from "../types";

export function useDisablesleep() {
  const [enabled, setEnabled] = useState(true);
  const [current, setCurrent] = useState<boolean | null>(null);
  const [remember, setRemember] = useState(true);
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<CommandResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const preview = `sudo pmset -a disablesleep ${enabled ? 1 : 0}`;

  useEffect(() => {
    api
      .getDisablesleep()
      .then((state) => {
        if (state.enabled !== null) {
          setCurrent(state.enabled);
          setEnabled(state.enabled);
        }
      })
      .catch(() => {
        // pmset -g が読めない環境では不明のままにする
      });
  }, []);

  async function execute() {
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await api.setDisablesleep(enabled);
      setResult(res);
      if (res.success) setCurrent(enabled);
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  async function updateRemember(next: boolean) {
    setRemember(next);
    await api.setRemember(next);
  }

  return {
    enabled,
    setEnabled,
    current,
    remember,
    executing,
    result,
    error,
    preview,
    execute,
    updateRemember,
  };
}
