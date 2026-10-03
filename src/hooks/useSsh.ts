import { useState } from "react";
import { api } from "../api";
import {
  SSH_DEFAULT_MODE,
  SSH_DEFAULT_STRICT,
  SSH_DEFAULT_TIMEOUT,
  isValidSshHost,
  isValidSshPort,
  isValidSshTimeout,
  isValidSshUser,
  type SshMode,
  type SshStrict,
  type SshTransport,
} from "../commands/sshOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { SshSnapshot } from "../types";
import { useRunner } from "./useRunner";

export function useSsh() {
  const [host, setHost] = useState("example.com");
  const [user, setUser] = useState("");
  const [port, setPort] = useState("");
  const [mode, setMode] = useState<SshMode>(SSH_DEFAULT_MODE);
  const [timeout, setTimeout] = useState(String(SSH_DEFAULT_TIMEOUT));
  const [strict, setStrict] = useState<SshStrict>(SSH_DEFAULT_STRICT);
  const [verbose, setVerbose] = useState(false);
  const [transport, setTransport] = useState<SshTransport>("auto");
  const runner = useRunner<SshSnapshot>();

  const hostOk = isValidSshHost(host);
  const userOk = isValidSshUser(user);
  const portOk = isValidSshPort(port);
  const timeoutOk = mode === "config" || isValidSshTimeout(timeout);
  const valid = hostOk && userOk && portOk && timeoutOk;

  const tokens = buildTokens({
    host: host.trim(),
    user: user.trim(),
    port: port.trim(),
    mode,
    timeout: timeout.trim(),
    strict,
    verbose,
    transport,
  });
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(
      () =>
        api.getSsh({
          host: host.trim(),
          user: user.trim(),
          port: port.trim() === "" ? null : Number(port),
          mode,
          connectTimeout: mode === "config" ? SSH_DEFAULT_TIMEOUT : Number(timeout),
          strict,
          verbose,
          transport,
        }),
      preview,
    );
  }

  return {
    host,
    setHost,
    user,
    setUser,
    port,
    setPort,
    mode,
    setMode,
    timeout,
    setTimeout,
    strict,
    setStrict,
    verbose,
    setVerbose,
    transport,
    setTransport,
    runner,
    tokens,
    preview,
    valid,
    hostOk,
    userOk,
    portOk,
    timeoutOk,
    execute,
  };
}

function destination(host: string, user: string): string {
  return user ? `${user}@${host}` : host;
}

function buildTokens(input: {
  host: string;
  user: string;
  port: string;
  mode: SshMode;
  timeout: string;
  strict: SshStrict;
  verbose: boolean;
  transport: SshTransport;
}): CmdToken[] {
  const out: CmdToken[] = [tok("ssh", "cmd")];
  if (input.transport === "4" || input.transport === "6") {
    out.push(tok(`-${input.transport}`, "flag", "transport"));
  }
  if (input.verbose) out.push(tok("-v", "flag", "verbose"));
  if (input.mode === "config") {
    out.push(tok("-T", "flag", "mode"));
    if (input.port) {
      out.push(tok("-p", "flag", "port"), tok(input.port, "value", "port"));
    }
    out.push(tok("-G", "flag", "mode"));
    out.push(
      tok(
        destination(input.host || "…", input.user),
        input.host ? "value" : "placeholder",
        "destination",
      ),
    );
  } else {
    out.push(tok("-T", "flag", "mode"));
    out.push(tok("-o", "flag", "batch"), tok("BatchMode=yes", "value", "batch"));
    out.push(tok("-o", "flag", "timeout"), tok(`ConnectTimeout=${input.timeout}`, "value", "timeout"));
    out.push(
      tok("-o", "flag", "strict"),
      tok(`StrictHostKeyChecking=${input.strict}`, "value", "strict"),
    );
    if (input.port) {
      out.push(tok("-p", "flag", "port"), tok(input.port, "value", "port"));
    }
    out.push(
      tok(
        destination(input.host || "…", input.user),
        input.host ? "value" : "placeholder",
        "destination",
      ),
    );
    out.push(tok("exit", "fixed", "remote"));
  }
  return out;
}
