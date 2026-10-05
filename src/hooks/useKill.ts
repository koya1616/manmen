import { useEffect, useState } from "react";
import { api } from "../api";
import { parseKillPids, type KillSignal } from "../commands/killOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { KillSnapshot, KillTarget } from "../types";
import { useRunner } from "./useRunner";

const LOOKUP_DELAY_MS = 300;

// 送信は「実行」→ 確認パネルの「送信する」の2段階。
// 入力を変えたら確認状態は解除し、古い対象に送らないようにする。
export function useKill() {
  const [pids, setPidsRaw] = useState("");
  const [signal, setSignalRaw] = useState<KillSignal>("TERM");
  const [sudo, setSudoRaw] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [targets, setTargets] = useState<KillTarget[]>([]);
  const [targetsError, setTargetsError] = useState<string | null>(null);
  const [lookupSeq, setLookupSeq] = useState(0);
  const runner = useRunner<KillSnapshot>();

  const parsed = parseKillPids(pids);
  const pidsOk = parsed !== null;
  const pidKey = parsed ? parsed.join(" ") : "";
  const anyFound = targets.some((target) => target.found);
  const valid = pidsOk && anyFound;

  // 入力が落ち着いたら対象プロセスを引き直す (権限不要の ps)。
  useEffect(() => {
    if (pidKey === "") {
      setTargets([]);
      setTargetsError(null);
      return;
    }
    let cancelled = false;
    const timer = window.setTimeout(() => {
      api
        .getKillTargets(pidKey)
        .then((next) => {
          if (cancelled) return;
          setTargets(next);
          setTargetsError(null);
        })
        .catch((e) => {
          if (cancelled) return;
          setTargets([]);
          setTargetsError(String(e));
        });
    }, LOOKUP_DELAY_MS);
    return () => {
      cancelled = true;
      window.clearTimeout(timer);
    };
  }, [pidKey, lookupSeq]);

  const tokens = buildTokens({ pids: parsed ?? [], signal, sudo });
  const preview = tokensToString(tokens);

  function setPids(next: string) {
    setPidsRaw(next);
    setConfirming(false);
  }

  function setSignal(next: KillSignal) {
    setSignalRaw(next);
    setConfirming(false);
  }

  function setSudo(next: boolean) {
    setSudoRaw(next);
    setConfirming(false);
  }

  function refreshTargets() {
    setLookupSeq((n) => n + 1);
  }

  function requestConfirm() {
    if (valid) setConfirming(true);
  }

  function cancelConfirm() {
    setConfirming(false);
  }

  async function execute() {
    if (!valid || !confirming) return;
    setConfirming(false);
    await runner.run(() => api.sendKill({ pids: pidKey, signal, sudo }), preview);
    refreshTargets();
  }

  return {
    pids,
    setPids,
    signal,
    setSignal,
    sudo,
    setSudo,
    confirming,
    requestConfirm,
    cancelConfirm,
    targets,
    targetsError,
    refreshTargets,
    runner,
    tokens,
    preview,
    valid,
    pidsOk,
    anyFound,
    execute,
  };
}

function buildTokens(input: { pids: string[]; signal: KillSignal; sudo: boolean }): CmdToken[] {
  const out: CmdToken[] = [];
  if (input.sudo) out.push(tok("sudo", "sudo", "sudo"));
  out.push(tok("kill", "cmd"));
  out.push(tok("-s", "flag", "signal"), tok(input.signal, "value", "signal"));
  if (input.pids.length === 0) {
    out.push(tok("PID", "placeholder", "pids"));
  } else {
    for (const pid of input.pids) out.push(tok(pid, "value", "pids"));
  }
  return out;
}
