import { useState } from "react";
import { api } from "../api";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { IfconfigSnapshot } from "../types";
import { useRunner } from "./useRunner";

export type IfconfigFamily = "" | "inet" | "inet6";

// Rust の ifconfig.rs の validate_interface と同じ形 (英小文字 + 数字)。
const INTERFACE_PATTERN = /^[a-z]{1,15}[0-9]{0,5}$/;

export function useIfconfig() {
  const [iface, setIface] = useState("");
  const [upOnly, setUpOnly] = useState(false);
  const [family, setFamily] = useState<IfconfigFamily>("");
  // 表示だけの絞り込み (コマンドには影響しない)
  const [activeOnly, setActiveOnly] = useState(true);
  const runner = useRunner<IfconfigSnapshot>();

  const ifaceOk = iface.trim() === "" || INTERFACE_PATTERN.test(iface.trim());
  const valid = ifaceOk;

  const query = { interface: iface.trim(), upOnly, family };
  const tokens = buildTokens(query);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(() => api.getIfconfig(query), preview);
  }

  return {
    iface,
    setIface,
    upOnly,
    setUpOnly,
    family,
    setFamily,
    activeOnly,
    setActiveOnly,
    runner,
    tokens,
    preview,
    valid,
    ifaceOk,
    execute,
  };
}

// Rust の Ifconfig::preview と同じ並びにすること。
function buildTokens(input: { interface: string; upOnly: boolean; family: IfconfigFamily }): CmdToken[] {
  const out: CmdToken[] = [tok("ifconfig", "cmd")];
  if (input.interface) {
    out.push(tok(input.interface, "value", "interface"));
  } else {
    out.push(tok("-a", "flag", "interface"));
    if (input.upOnly) out.push(tok("-u", "flag", "upOnly"));
  }
  if (input.family) out.push(tok(input.family, "value", "family"));
  return out;
}
