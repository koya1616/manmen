import { useState } from "react";
import { api } from "../api";
import {
  PS_DEFAULT_COLUMNS,
  PS_DEFAULT_SORT,
  PS_MAX_COLUMNS,
  isValidUser,
  parsePidList,
  type PsSort,
} from "../commands/psOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { PsSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function usePs() {
  const [sort, setSort] = useState<PsSort>(PS_DEFAULT_SORT);
  const [columns, setColumns] = useState<string[]>([...PS_DEFAULT_COLUMNS]);
  const [user, setUser] = useState("");
  const [pids, setPids] = useState("");
  const runner = useRunner<PsSnapshot>();

  const userOk = isValidUser(user);
  const pidList = parsePidList(pids);
  const columnsOk = columns.length >= 1 && columns.length <= PS_MAX_COLUMNS;
  const valid = userOk && pidList !== null && columnsOk;

  const tokens = buildTokens({
    sort,
    columns,
    user: user.trim(),
    pids: pidList ?? [],
  });
  const preview = tokensToString(tokens);

  function toggleColumn(key: string) {
    setColumns((current) =>
      current.includes(key)
        ? current.filter((item) => item !== key)
        : current.length >= PS_MAX_COLUMNS
          ? current
          : [...current, key],
    );
  }

  async function execute() {
    if (!valid || pidList === null) return;
    await runner.run(
      () =>
        api.getPs({
          sort,
          columns,
          user: user.trim(),
          pids: pidList.join(","),
        }),
      preview,
    );
  }

  function resetColumns() {
    setColumns([...PS_DEFAULT_COLUMNS]);
  }

  return {
    sort,
    setSort,
    columns,
    toggleColumn,
    resetColumns,
    user,
    setUser,
    pids,
    setPids,
    runner,
    tokens,
    preview,
    valid,
    userOk,
    pidList,
    columnsOk,
    execute,
  };
}

function buildTokens(input: {
  sort: PsSort;
  columns: string[];
  user: string;
  pids: string[];
}): CmdToken[] {
  const out: CmdToken[] = [tok("ps", "cmd"), tok("-A", "flag", "all")];
  if (input.sort === "cpu") out.push(tok("-r", "flag", "sort"));
  if (input.sort === "mem") out.push(tok("-m", "flag", "sort"));
  if (input.user) out.push(tok("-U", "flag", "user"), tok(input.user, "value", "user"));
  if (input.pids.length > 0) {
    out.push(tok("-p", "flag", "pids"), tok(input.pids.join(","), "value", "pids"));
  }
  // 空白を含む command は末尾に回す (Rust 側と同じ順序にする)
  const head = input.columns.filter((column) => column !== "command");
  const tail = input.columns.filter((column) => column === "command");
  const ordered = [...head, ...tail];
  out.push(
    tok("-o", "flag", "columns"),
    tok(ordered.join(",") || "…", ordered.length > 0 ? "value" : "placeholder", "columns"),
  );
  return out;
}
