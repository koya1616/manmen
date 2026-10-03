import { useState } from "react";
import { api } from "../api";
import {
  LSOF_DEFAULT_PROTOCOL,
  LSOF_DEFAULT_STATE,
  isValidComm,
  isValidHost,
  isValidPort,
  isValidUser,
  parsePidList,
  type LsofProtocol,
  type LsofState,
} from "../commands/lsofOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { LsofSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useLsof() {
  const [pids, setPids] = useState("");
  const [user, setUser] = useState("");
  const [comm, setComm] = useState("");
  const [protocol, setProtocol] = useState<LsofProtocol>(LSOF_DEFAULT_PROTOCOL);
  const [port, setPort] = useState("");
  const [host, setHost] = useState("");
  const [state, setState] = useState<LsofState>(LSOF_DEFAULT_STATE);
  const runner = useRunner<LsofSnapshot>();

  const pidList = parsePidList(pids);
  const userOk = isValidUser(user);
  const commOk = isValidComm(comm);
  const portOk = isValidPort(port);
  const hostOk = isValidHost(host);
  const stateConflict = protocol === "UDP" && state !== "any";
  // 結果が膨大になるため、絞り込みなしでは実行できない
  const hasSelector =
    (pidList !== null && pidList.length > 0) ||
    user.trim() !== "" ||
    comm.trim() !== "" ||
    protocol !== "any" ||
    port.trim() !== "" ||
    host.trim() !== "" ||
    state !== "any";
  const valid =
    pidList !== null && userOk && commOk && portOk && hostOk && !stateConflict && hasSelector;

  const tokens = buildTokens({
    pids: pidList ?? [],
    user: user.trim(),
    comm: comm.trim(),
    protocol,
    port: port.trim(),
    host: host.trim(),
    state,
  });
  const preview = tokensToString(tokens);

  function pickProtocol(next: LsofProtocol) {
    setProtocol(next);
    // UDP に状態の絞り込みは付けられない
    if (next === "UDP") setState("any");
  }

  async function execute() {
    if (!valid || pidList === null) return;
    await runner.run(
      () =>
        api.getLsof({
          pids: pidList.join(","),
          user: user.trim(),
          comm: comm.trim(),
          protocol,
          port: port.trim(),
          host: host.trim(),
          state,
        }),
      preview,
    );
  }

  return {
    pids,
    setPids,
    user,
    setUser,
    comm,
    setComm,
    protocol,
    pickProtocol,
    port,
    setPort,
    host,
    setHost,
    state,
    setState,
    runner,
    tokens,
    preview,
    valid,
    pidList,
    userOk,
    commOk,
    portOk,
    hostOk,
    stateConflict,
    hasSelector,
    execute,
  };
}

function buildTokens(input: {
  pids: string[];
  user: string;
  comm: string;
  protocol: LsofProtocol;
  port: string;
  host: string;
  state: LsofState;
}): CmdToken[] {
  const out: CmdToken[] = [
    tok("lsof", "cmd"),
    tok("-a", "flag", "and"),
    tok("-P", "flag", "noResolve"),
    tok("-n", "flag", "noResolve"),
    tok("+c", "flag", "fullName"),
    tok("0", "value", "fullName"),
  ];
  if (input.pids.length > 0) {
    out.push(tok("-p", "flag", "pids"), tok(input.pids.join(","), "value", "pids"));
  }
  if (input.user) out.push(tok("-u", "flag", "user"), tok(input.user, "value", "user"));
  if (input.comm) out.push(tok("-c", "flag", "comm"), tok(input.comm, "value", "comm"));
  const netActive =
    input.protocol !== "any" || input.port !== "" || input.host !== "" || input.state !== "any";
  if (netActive) {
    let spec = "-i";
    if (input.protocol !== "any") spec += input.protocol;
    if (input.host) spec += `@${input.host}`;
    if (input.port) spec += `:${input.port}`;
    out.push(tok(spec, "value", "network"));
    if (input.state !== "any") {
      out.push(tok(`-sTCP:${input.state}`, "value", "network"));
    }
  }
  if (out.length === 6) {
    out.push(tok("…", "placeholder", "selector"));
  }
  return out;
}
