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
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { TopSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useTop() {
  const [sortKey, setSortKey] = useState<TopKey>(TOP_DEFAULT_SORT);
  const [sortOrder, setSortOrder] = useState<TopSortOrder>("");
  const [secondaryKey, setSecondaryKey] = useState("");
  const [count, setCount] = useState(String(TOP_DEFAULT_COUNT));
  const [countMode, setCountMode] = useState<TopCountMode>("n");
  const [noFrameworks, setNoFrameworks] = useState(false);
  const [memoryMap, setMemoryMap] = useState(false);
  const [swap, setSwap] = useState(false);
  const [user, setUser] = useState("");
  const [pids, setPids] = useState("");
  const [stats, setStats] = useState<string[]>([]);
  const [ncols, setNcols] = useState("");
  const runner = useRunner<TopSnapshot>();

  const countNum = Number(count);
  const isValidCount =
    /^\d+$/.test(count) && countNum >= 1 && countNum <= TOP_MAX_COUNT;
  const userOk = isValidUser(user);
  const pidList = parsePidList(pids);
  const ncolsOk = isValidNcols(ncols);
  const statsOk = stats.length <= TOP_MAX_STATS;
  const valid = isValidCount && userOk && pidList !== null && ncolsOk && statsOk;

  const tokens = buildTokens({
    sortKey,
    sortOrder,
    secondaryKey,
    count: isValidCount ? String(countNum) : "",
    countMode,
    noFrameworks,
    memoryMap,
    swap,
    user: user.trim(),
    pids: pidList ?? [],
    stats,
    ncols: ncols.trim(),
  });
  const preview = tokensToString(tokens);

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
    await runner.run(
      () =>
        api.getTop({
          sortKey,
          sortOrder,
          secondaryKey,
          count: countNum,
          countMode,
          noFrameworks,
          memoryMap,
          swap,
          user: user.trim(),
          pids: pidList.join(","),
          stats,
          ncols: ncols.trim() === "" ? null : Number(ncols),
        }),
      preview,
    );
  }

  function clearStats() {
    setStats([]);
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
    clearStats,
    ncols,
    setNcols,
    runner,
    tokens,
    preview,
    valid,
    isValidCount,
    userOk,
    pidList,
    ncolsOk,
    execute,
  };
}

function buildTokens(input: {
  sortKey: string;
  sortOrder: string;
  secondaryKey: string;
  count: string;
  countMode: string;
  noFrameworks: boolean;
  memoryMap: boolean;
  swap: boolean;
  user: string;
  pids: string[];
  stats: string[];
  ncols: string;
}): CmdToken[] {
  const out: CmdToken[] = [tok("top", "cmd"), tok("-l", "fixed"), tok("1", "fixed")];
  out.push(tok("-o", "flag", "sort"), tok(`${input.sortOrder}${input.sortKey}`, "value", "sort"));
  if (input.secondaryKey) {
    out.push(tok("-O", "flag", "secondary"), tok(input.secondaryKey, "value", "secondary"));
  }
  out.push(tok("-n", "flag", "count"), tok(input.count || "…", input.count ? "value" : "placeholder", "count"));
  if (input.countMode !== "n") {
    out.push(tok("-c", "flag", "mode"), tok(input.countMode, "value", "mode"));
  }
  if (input.noFrameworks) out.push(tok("-F", "flag", "frameworks"));
  if (input.memoryMap) out.push(tok("-r", "flag", "memoryMap"));
  if (input.swap) out.push(tok("-S", "flag", "swap"));
  if (input.user) out.push(tok("-user", "flag", "user"), tok(input.user, "value", "user"));
  for (const pid of input.pids) out.push(tok("-pid", "flag", "pids"), tok(pid, "value", "pids"));
  if (input.stats.length > 0) {
    out.push(tok("-stats", "flag", "stats"), tok(input.stats.join(","), "value", "stats"));
  }
  if (input.ncols) out.push(tok("-ncols", "flag", "ncols"), tok(input.ncols, "value", "ncols"));
  return out;
}
