import { useState } from "react";
import { api } from "../api";
import {
  TOP_DEFAULT_COUNT,
  TOP_DEFAULT_SORT,
  TOP_MAX_COUNT,
  TOP_MAX_STATS,
  isValidNcols,
  isValidUser,
  parsePidList,
  type TopCountMode,
  type TopKey,
  type TopSortOrder,
} from "../commands/topOptions";
import type { TopSnapshot } from "../types";

export function useTop() {
  const [sortKey, setSortKey] = useState<TopKey>(TOP_DEFAULT_SORT);
  const [sortOrder, setSortOrder] = useState<TopSortOrder>("");
  const [secondaryKey, setSecondaryKey] = useState("");
  const [count, setCount] = useState(TOP_DEFAULT_COUNT);
  const [countMode, setCountMode] = useState<TopCountMode>("n");
  const [noFrameworks, setNoFrameworks] = useState(false);
  const [memoryMap, setMemoryMap] = useState(false);
  const [swap, setSwap] = useState(false);
  const [user, setUser] = useState("");
  const [pids, setPids] = useState("");
  const [stats, setStats] = useState<string[]>([]);
  const [ncols, setNcols] = useState("");
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<TopSnapshot | null>(null);
  const [error, setError] = useState<string | null>(null);

  const isValidCount =
    Number.isInteger(count) && count >= 1 && count <= TOP_MAX_COUNT;
  const userOk = isValidUser(user);
  const pidList = parsePidList(pids);
  const ncolsOk = isValidNcols(ncols);
  const statsOk = stats.length <= TOP_MAX_STATS;
  const valid = isValidCount && userOk && pidList !== null && ncolsOk && statsOk;

  const preview = buildPreview({
    sortKey,
    sortOrder,
    secondaryKey,
    count,
    countMode,
    noFrameworks,
    memoryMap,
    swap,
    user: user.trim(),
    pids: pidList ?? [],
    stats,
    ncols: ncols.trim(),
  });

  function toggleStat(key: string) {
    setStats((current) =>
      current.includes(key)
        ? current.filter((item) => item !== key)
        : current.length >= TOP_MAX_STATS
          ? current
          : [...current, key],
    );
  }

  async function execute() {
    if (!valid || pidList === null) return;
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await api.getTop({
        sortKey,
        sortOrder,
        secondaryKey,
        count,
        countMode,
        noFrameworks,
        memoryMap,
        swap,
        user: user.trim(),
        pids: pidList.join(","),
        stats,
        ncols: ncols.trim() === "" ? null : Number(ncols),
      });
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
    sortOrder,
    setSortOrder,
    secondaryKey,
    setSecondaryKey,
    count,
    setCount,
    countMode,
    setCountMode,
    noFrameworks,
    setNoFrameworks,
    memoryMap,
    setMemoryMap,
    swap,
    setSwap,
    user,
    setUser,
    pids,
    setPids,
    stats,
    toggleStat,
    ncols,
    setNcols,
    executing,
    result,
    error,
    preview,
    valid,
    userOk,
    pidList,
    ncolsOk,
    execute,
  };
}

function buildPreview(input: {
  sortKey: string;
  sortOrder: string;
  secondaryKey: string;
  count: number;
  countMode: string;
  noFrameworks: boolean;
  memoryMap: boolean;
  swap: boolean;
  user: string;
  pids: string[];
  stats: string[];
  ncols: string;
}): string {
  const args = ["-l", "1", "-o", `${input.sortOrder}${input.sortKey}`];
  if (input.secondaryKey) args.push("-O", input.secondaryKey);
  args.push("-n", String(input.count));
  if (input.countMode !== "n") args.push("-c", input.countMode);
  if (input.noFrameworks) args.push("-F");
  if (input.memoryMap) args.push("-r");
  if (input.swap) args.push("-S");
  if (input.user) args.push("-user", input.user);
  for (const pid of input.pids) args.push("-pid", pid);
  if (input.stats.length > 0) args.push("-stats", input.stats.join(","));
  if (input.ncols) args.push("-ncols", input.ncols);
  return `top ${args.join(" ")}`;
}
