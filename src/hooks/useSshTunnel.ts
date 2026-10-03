import { useCallback, useEffect, useState } from "react";
import { api } from "../api";
import {
  TUNNEL_MODES,
  isValidRequiredPort,
  isValidTunnelBind,
  isValidTunnelRemoteHost,
  isValidTunnelRemotePort,
  isValidTunnelSshHost,
  isValidTunnelSshPort,
  isValidTunnelSshUser,
  type TunnelMode,
} from "../commands/sshTunnelOptions";
import type { SshTransport } from "../commands/sshOptions";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { TunnelInfo } from "../types";

export function useSshTunnel(active: boolean) {
  const [tunnelMode, setTunnelMode] = useState<TunnelMode>(TUNNEL_MODES[0]);
  const [localHost, setLocalHost] = useState("");
  const [localPort, setLocalPort] = useState("8080");
  const [remoteHost, setRemoteHost] = useState("");
  const [remotePort, setRemotePort] = useState("");
  const [host, setHost] = useState("");
  const [user, setUser] = useState("");
  const [port, setPort] = useState("");
  const [keepalive, setKeepalive] = useState(true);
  const [transport, setTransport] = useState<SshTransport>("auto");
  const [tunnels, setTunnels] = useState<TunnelInfo[]>([]);
  const [starting, setStarting] = useState(false);
  const [stoppingId, setStoppingId] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const bindOk = isValidTunnelBind(localHost);
  const localPortOk = isValidRequiredPort(localPort);
  const remoteHostOk = isValidTunnelRemoteHost(remoteHost, tunnelMode);
  const remotePortOk = isValidTunnelRemotePort(remotePort, tunnelMode);
  const hostOk = isValidTunnelSshHost(host);
  const userOk = isValidTunnelSshUser(user);
  const portOk = isValidTunnelSshPort(port);
  const valid =
    bindOk && localPortOk && remoteHostOk && remotePortOk && hostOk && userOk && portOk;

  const tokens = buildTokens({
    tunnelMode,
    localHost: localHost.trim(),
    localPort: localPort.trim(),
    remoteHost: remoteHost.trim(),
    remotePort: remotePort.trim(),
    host: host.trim(),
    user: user.trim(),
    port: port.trim(),
    keepalive,
    transport,
  });
  const preview = tokensToString(tokens);

  const refresh = useCallback(async () => {
    try {
      setTunnels(await api.listSshTunnels());
    } catch {
      // 一覧の取得失敗は無視する (次回ポーリングで復帰する)
    }
  }, []);

  useEffect(() => {
    if (!active) return;
    refresh();
    const timer = window.setInterval(refresh, 5000);
    return () => window.clearInterval(timer);
  }, [active, refresh]);

  async function start() {
    if (!valid || starting) return;
    setStarting(true);
    setError(null);
    try {
      await api.startSshTunnel({
        mode: tunnelMode,
        localHost: localHost.trim(),
        localPort: Number(localPort),
        remoteHost: remoteHost.trim(),
        remotePort:
          tunnelMode === "dynamic" || remotePort.trim() === ""
            ? null
            : Number(remotePort),
        host: host.trim(),
        user: user.trim(),
        port: port.trim() === "" ? null : Number(port),
        keepalive,
        transport,
      });
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setStarting(false);
    }
  }

  async function stop(id: string) {
    setStoppingId(id);
    try {
      await api.stopSshTunnel(id);
      await refresh();
    } catch (e) {
      setError(String(e));
    } finally {
      setStoppingId(null);
    }
  }

  return {
    tunnelMode,
    setTunnelMode,
    localHost,
    setLocalHost,
    localPort,
    setLocalPort,
    remoteHost,
    setRemoteHost,
    remotePort,
    setRemotePort,
    host,
    setHost,
    user,
    setUser,
    port,
    setPort,
    keepalive,
    setKeepalive,
    transport,
    setTransport,
    tunnels,
    starting,
    stoppingId,
    error,
    refresh,
    tokens,
    preview,
    valid,
    bindOk,
    localPortOk,
    remoteHostOk,
    remotePortOk,
    hostOk,
    userOk,
    portOk,
    start,
    stop,
  };
}

function destination(host: string, user: string): string {
  return user ? `${user}@${host}` : host;
}

function forwardSpec(input: {
  tunnelMode: TunnelMode;
  localHost: string;
  localPort: string;
  remoteHost: string;
  remotePort: string;
}): { flag: string; spec: string } {
  const bind = input.localHost === "" || input.localHost === "localhost" ? "127.0.0.1" : input.localHost;
  if (input.tunnelMode === "dynamic") {
    return { flag: "-D", spec: `${bind}:${input.localPort || "…"}` };
  }
  const flag = input.tunnelMode === "local" ? "-L" : "-R";
  const spec = `${bind}:${input.localPort || "…"}:${input.remoteHost || "…"}:${input.remotePort || "…"}`;
  return { flag, spec };
}

function buildTokens(input: {
  tunnelMode: TunnelMode;
  localHost: string;
  localPort: string;
  remoteHost: string;
  remotePort: string;
  host: string;
  user: string;
  port: string;
  keepalive: boolean;
  transport: SshTransport;
}): CmdToken[] {
  const out: CmdToken[] = [tok("ssh", "cmd")];
  out.push(tok("-N", "flag", "tunnelMode"), tok("-T", "flag", "tunnelMode"));
  out.push(tok("-o", "flag", "batch"), tok("BatchMode=yes", "value", "batch"));
  out.push(
    tok("-o", "flag", "forward"),
    tok("ExitOnForwardFailure=yes", "value", "forward"),
  );
  if (input.keepalive) {
    out.push(tok("-o", "flag", "keepalive"), tok("ServerAliveInterval=30", "value", "keepalive"));
    out.push(tok("-o", "flag", "keepalive"), tok("ServerAliveCountMax=3", "value", "keepalive"));
  }
  if (input.transport === "4" || input.transport === "6") {
    out.push(tok(`-${input.transport}`, "flag", "transport"));
  }
  if (input.port) {
    out.push(tok("-p", "flag", "port"), tok(input.port, "value", "port"));
  }
  const { flag, spec } = forwardSpec(input);
  out.push(tok(flag, "flag", "forward"), tok(spec, "value", "forward"));
  out.push(
    tok(
      destination(input.host || "…", input.user),
      input.host ? "value" : "placeholder",
      "destination",
    ),
  );
  return out;
}
