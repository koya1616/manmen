import { useState } from "react";
import { api } from "../api";
import { isValidPingHost } from "../commands/pingOptions";
import {
  TRACEROUTE_DEFAULT_MAX_TTL,
  TRACEROUTE_DEFAULT_QUERIES,
  TRACEROUTE_DEFAULT_WAIT,
  TRACEROUTE_MAX_WORST_SECS,
  isValidTracerouteFirstTtl,
  isValidTracerouteMaxTtl,
  isValidTracerouteQueries,
  isValidTracerouteWait,
  tracerouteWorstSecs,
} from "../commands/tracerouteOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { TracerouteSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useTraceroute() {
  const [host, setHost] = useState("example.com");
  const [maxTtl, setMaxTtl] = useState(String(TRACEROUTE_DEFAULT_MAX_TTL));
  const [firstTtl, setFirstTtl] = useState("");
  const [queries, setQueries] = useState(String(TRACEROUTE_DEFAULT_QUERIES));
  const [wait, setWait] = useState(String(TRACEROUTE_DEFAULT_WAIT));
  const [icmp, setIcmp] = useState(false);
  const [numeric, setNumeric] = useState(false);
  const [asLookup, setAsLookup] = useState(false);
  const runner = useRunner<TracerouteSnapshot>();

  const hostOk = isValidPingHost(host);
  const maxTtlOk = isValidTracerouteMaxTtl(maxTtl);
  const maxTtlNum = maxTtlOk ? Number(maxTtl.trim()) : TRACEROUTE_DEFAULT_MAX_TTL;
  const firstTtlOk = isValidTracerouteFirstTtl(firstTtl, maxTtlNum);
  const queriesOk = isValidTracerouteQueries(queries);
  const waitOk = isValidTracerouteWait(wait);

  const firstTtlNum = firstTtl.trim() === "" || !firstTtlOk ? null : Number(firstTtl.trim());
  const queriesNum = queriesOk ? Number(queries.trim()) : TRACEROUTE_DEFAULT_QUERIES;
  const waitNum = waitOk ? Number(wait.trim()) : TRACEROUTE_DEFAULT_WAIT;
  const worstSecs = tracerouteWorstSecs({
    maxTtl: maxTtlNum,
    firstTtl: firstTtlNum,
    queries: queriesNum,
    wait: waitNum,
  });
  const worstOk = worstSecs <= TRACEROUTE_MAX_WORST_SECS;
  const valid = hostOk && maxTtlOk && firstTtlOk && queriesOk && waitOk && worstOk;

  const query = {
    host: host.trim(),
    maxTtl: maxTtlNum,
    firstTtl: firstTtlNum,
    queries: queriesNum,
    wait: waitNum,
    icmp,
    numeric,
    asLookup,
  };
  const tokens = buildTokens(query);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getTraceroute(query), preview);
  }

  return {
    host,
    setHost,
    maxTtl,
    setMaxTtl,
    firstTtl,
    setFirstTtl,
    queries,
    setQueries,
    wait,
    setWait,
    icmp,
    setIcmp,
    numeric,
    setNumeric,
    asLookup,
    setAsLookup,
    runner,
    tokens,
    preview,
    valid,
    hostOk,
    maxTtlOk,
    firstTtlOk,
    queriesOk,
    waitOk,
    worstOk,
    worstSecs,
    execute,
  };
}

// Rust の Traceroute::preview と同じ並びにすること。
function buildTokens(input: {
  host: string;
  maxTtl: number;
  firstTtl: number | null;
  queries: number;
  wait: number;
  icmp: boolean;
  numeric: boolean;
  asLookup: boolean;
}): CmdToken[] {
  const out: CmdToken[] = [tok("traceroute", "cmd")];
  out.push(tok("-m", "flag", "maxTtl"), tok(String(input.maxTtl), "value", "maxTtl"));
  if (input.firstTtl !== null) {
    out.push(tok("-f", "flag", "firstTtl"), tok(String(input.firstTtl), "value", "firstTtl"));
  }
  out.push(tok("-q", "flag", "queries"), tok(String(input.queries), "value", "queries"));
  out.push(tok("-w", "flag", "wait"), tok(String(input.wait), "value", "wait"));
  if (input.icmp) out.push(tok("-I", "flag", "protocol"));
  if (input.numeric) out.push(tok("-n", "flag", "numeric"));
  if (input.asLookup) out.push(tok("-a", "flag", "asLookup"));
  out.push(tok(input.host || "…", input.host ? "value" : "placeholder", "host"));
  return out;
}
