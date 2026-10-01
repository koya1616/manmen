import { useState } from "react";
import { api } from "../api";
import type { TopSnapshot } from "../types";

export const TOP_SORT_KEYS = ["cpu", "mem", "time", "pid", "command"] as const;
export type TopSortKey = (typeof TOP_SORT_KEYS)[number];

export const TOP_DEFAULT_SORT: TopSortKey = "cpu";
export const TOP_DEFAULT_COUNT = 20;
export const TOP_MAX_COUNT = 100;

export function useTop() {
  const [sortKey, setSortKey] = useState<TopSortKey>(TOP_DEFAULT_SORT);
  const [count, setCount] = useState(TOP_DEFAULT_COUNT);
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<TopSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);

  const preview = `top -l 1 -o ${sortKey} -n ${count}`;
  const isValidCount =
    Number.isInteger(count) && count >= 1 && count <= TOP_MAX_COUNT;

  async function execute() {
    if (!isValidCount) return;
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await api.getTop(sortKey, count);
      setResult(res);
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  return {
    sortKey,
    setSortKey,
    count,
    setCount,
    executing,
    result,
    error,
    preview,
    isValidCount,
    execute,
  };
}
