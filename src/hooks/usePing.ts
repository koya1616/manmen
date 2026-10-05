import { useState } from "react";
import { api } from "../api";
import {
  PING_DEFAULT_COUNT,
  PING_DEFAULT_INTERVAL,
  PING_DEFAULT_SIZE,
  isValidPingCount,
  isValidPingHost,
  isValidPingInterval,
  isValidPingSize,
  isValidPingTimeout,
  isValidPingTtl,
  isValidPingWaitMs,
} from "../commands/pingOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { PingSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function usePing() {
  const [host, setHost] = useState("example.com");
  const [count, setCount] = useState(String(PING_DEFAULT_COUNT));
  const [interval, setInterval] = useState(String(PING_DEFAULT_INTERVAL));
  const [timeout, setTimeout] = useState("");
  const [waitMs, setWaitMs] = useState("");
  const [size, setSize] = useState(String(PING_DEFAULT_SIZE));
  const [ttl, setTtl] = useState("");
  const [numeric, setNumeric] = useState(false);
  const [noFragment, setNoFragment] = useState(false);
  const runner = useRunner<PingSnapshot>();

  const hostOk = isValidPingHost(host);
  const countOk = isValidPingCount(count);
  const intervalOk = isValidPingInterval(interval);
  const timeoutOk = isValidPingTimeout(timeout);
  const waitOk = isValidPingWaitMs(waitMs);
  const sizeOk = isValidPingSize(size);
  const ttlOk = isValidPingTtl(ttl);
  const valid = hostOk && countOk && intervalOk && timeoutOk && waitOk && sizeOk && ttlOk;

  const countNum = countOk ? Number(count.trim()) : PING_DEFAULT_COUNT;
  const intervalNum = intervalOk ? Number(interval.trim()) : PING_DEFAULT_INTERVAL;
  const timeoutNum = timeout.trim() === "" ? null : Number(timeout.trim());
  const waitNum = waitMs.trim() === "" ? null : Number(waitMs.trim());
  const sizeNum = sizeOk ? Number(size.trim()) : PING_DEFAULT_SIZE;
  const ttlNum = ttl.trim() === "" ? null : Number(ttl.trim());

  const tokens = buildTokens({
    host: host.trim(),
    count: countNum,
    interval: intervalNum,
    timeout: timeoutNum,
    waitMs: waitNum,
    size: sizeNum,
    ttl: ttlNum,
    numeric,
    noFragment,
  });
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(
      () =>
        api.getPing({
          host: host.trim(),
          count: countNum,
          interval: intervalNum,
          timeout: timeoutNum,
          waitMs: waitNum,
          size: sizeNum,
          ttl: ttlNum,
          numeric,
          noFragment,
        }),
      preview,
    );
  }

  return {
    host,
    setHost,
    count,
    setCount,
    interval,
    setInterval,
    timeout,
    setTimeout,
    waitMs,
    setWaitMs,
    size,
    setSize,
    ttl,
    setTtl,
    numeric,
    setNumeric,
    noFragment,
    setNoFragment,
    runner,
    tokens,
    preview,
    valid,
    hostOk,
    countOk,
    intervalOk,
    timeoutOk,
    waitOk,
    sizeOk,
    ttlOk,
    execute,
  };
}

function buildTokens(input: {
  host: string;
  count: number;
  interval: number;
  timeout: number | null;
  waitMs: number | null;
  size: number;
  ttl: number | null;
  numeric: boolean;
  noFragment: boolean;
}): CmdToken[] {
  const out: CmdToken[] = [tok("ping", "cmd")];
  out.push(tok("-c", "flag", "count"), tok(String(input.count), "value", "count"));
  if (input.interval !== PING_DEFAULT_INTERVAL) {
    out.push(tok("-i", "flag", "interval"), tok(String(input.interval), "value", "interval"));
  }
  if (input.timeout !== null) {
    out.push(tok("-t", "flag", "timeout"), tok(String(input.timeout), "value", "timeout"));
  }
  if (input.waitMs !== null) {
    out.push(tok("-W", "flag", "waitMs"), tok(String(input.waitMs), "value", "waitMs"));
  }
  if (input.size !== PING_DEFAULT_SIZE) {
    out.push(tok("-s", "flag", "size"), tok(String(input.size), "value", "size"));
  }
  if (input.ttl !== null) {
    out.push(tok("-m", "flag", "ttl"), tok(String(input.ttl), "value", "ttl"));
  }
  if (input.numeric) out.push(tok("-n", "flag", "numeric"));
  if (input.noFragment) out.push(tok("-D", "flag", "noFragment"));
  out.push(tok(input.host || "…", input.host ? "value" : "placeholder", "host"));
  return out;
}
