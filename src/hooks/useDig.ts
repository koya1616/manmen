import { useState } from "react";
import { api } from "../api";
import {
  DIG_DEFAULT_CLASS,
  DIG_DEFAULT_TYPE,
  isValidDigName,
  isValidDigServer,
  type DigClass,
  type DigTransport,
  type DigType,
} from "../commands/digOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { DigSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useDig() {
  const [name, setName] = useState("example.com");
  const [qtype, setQtype] = useState<DigType>(DIG_DEFAULT_TYPE);
  const [qclass, setQclass] = useState<DigClass>(DIG_DEFAULT_CLASS);
  const [server, setServer] = useState("");
  const [short, setShort] = useState(true);
  const [tcp, setTcp] = useState(false);
  const [dnssec, setDnssec] = useState(false);
  const [trace, setTrace] = useState(false);
  const [noRecurse, setNoRecurse] = useState(false);
  const [reverse, setReverse] = useState(false);
  const [transport, setTransport] = useState<DigTransport>("auto");
  const runner = useRunner<DigSnapshot>();

  const nameOk = isValidDigName(name, reverse);
  const serverOk = isValidDigServer(server);
  const traceConflict = trace && server.trim() !== "";
  const valid = nameOk && serverOk && !traceConflict;

  const tokens = buildTokens({
    name: name.trim(),
    qtype,
    qclass,
    server: server.trim(),
    short,
    tcp,
    dnssec,
    trace,
    noRecurse,
    reverse,
    transport,
  });
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    const trimmedServer = server.trim();
    const bare = trimmedServer.startsWith("@") ? trimmedServer.slice(1) : trimmedServer;
    await runner.run(
      () =>
        api.getDig({
          name: name.trim(),
          qtype,
          qclass,
          server: bare,
          short,
          tcp,
          dnssec,
          trace,
          noRecurse,
          reverse,
          transport,
        }),
      preview,
    );
  }

  return {
    name,
    setName,
    qtype,
    setQtype,
    qclass,
    setQclass,
    server,
    setServer,
    short,
    setShort,
    tcp,
    setTcp,
    dnssec,
    setDnssec,
    trace,
    setTrace,
    noRecurse,
    setNoRecurse,
    reverse,
    setReverse,
    transport,
    setTransport,
    runner,
    tokens,
    preview,
    valid,
    nameOk,
    serverOk,
    traceConflict,
    execute,
  };
}

function buildTokens(input: {
  name: string;
  qtype: string;
  qclass: string;
  server: string;
  short: boolean;
  tcp: boolean;
  dnssec: boolean;
  trace: boolean;
  noRecurse: boolean;
  reverse: boolean;
  transport: DigTransport;
}): CmdToken[] {
  const out: CmdToken[] = [tok("dig", "cmd")];
  if (input.transport === "4" || input.transport === "6") {
    out.push(tok(`-${input.transport}`, "flag", "transport"));
  }
  if (input.server) {
    const bare = input.server.startsWith("@") ? input.server : `@${input.server}`;
    out.push(tok(bare, "value", "server"));
  }
  if (input.reverse) {
    out.push(tok("-x", "flag", "reverse"), tok(input.name || "…", input.name ? "value" : "placeholder", "name"));
  } else {
    out.push(tok(input.name || "…", input.name ? "value" : "placeholder", "name"));
    out.push(tok(input.qtype, "value", "qtype"));
    if (input.qclass !== "IN") {
      out.push(tok(input.qclass, "value", "qclass"));
    }
  }
  if (input.short) out.push(tok("+short", "flag", "short"));
  if (input.tcp) out.push(tok("+tcp", "flag", "tcp"));
  if (input.dnssec) out.push(tok("+dnssec", "flag", "dnssec"));
  if (input.trace) out.push(tok("+trace", "flag", "trace"));
  if (input.noRecurse) out.push(tok("+norecurse", "flag", "noRecurse"));
  return out;
}
