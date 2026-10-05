import { useEffect, useState } from "react";
import { api } from "../api";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { NetworksetupSnapshot } from "../types";
import { useRunner } from "./useRunner";

export type NetworksetupSub = "services" | "info" | "wifi" | "locations";

export const NETWORKSETUP_SUBS: NetworksetupSub[] = ["services", "info", "wifi", "locations"];

// Rust の networksetup.rs の validate_device と同じ形 (英小文字 + 数字)。
const DEVICE_PATTERN = /^[a-z]+[0-9]+$/;

export function useNetworksetup() {
  const [sub, setSub] = useState<NetworksetupSub>("services");
  const [service, setService] = useState("");
  const [device, setDevice] = useState("");
  const [services, setServices] = useState<string[]>([]);
  const [servicesError, setServicesError] = useState<string | null>(null);
  const runner = useRunner<NetworksetupSnapshot>();

  // サービス詳細の選択肢は実在するサービス名だけにする (Rust 側でも同じ一覧で検証する)。
  useEffect(() => {
    let cancelled = false;
    api
      .listNetworkServices()
      .then((names) => {
        if (cancelled) return;
        setServices(names);
        setService((current) => current || (names.includes("Wi-Fi") ? "Wi-Fi" : (names[0] ?? "")));
      })
      .catch((e) => {
        if (!cancelled) setServicesError(String(e));
      });
    return () => {
      cancelled = true;
    };
  }, []);

  const serviceOk = sub !== "info" || services.includes(service);
  const deviceOk = sub !== "wifi" || device.trim() === "" || DEVICE_PATTERN.test(device.trim());
  const valid = serviceOk && deviceOk;

  const tokens = buildTokens(sub, service, device.trim());
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    await runner.run(
      () => api.getNetworksetup({ sub, service, device: device.trim() }),
      preview,
    );
  }

  return {
    sub,
    setSub,
    service,
    setService,
    device,
    setDevice,
    services,
    servicesError,
    runner,
    tokens,
    preview,
    valid,
    serviceOk,
    deviceOk,
    execute,
  };
}

// 表示するのは各サブコマンドの代表のコマンド。続けて実行する取得系は結果の「実行したコマンド」に出す。
function buildTokens(sub: NetworksetupSub, service: string, device: string): CmdToken[] {
  const out: CmdToken[] = [tok("networksetup", "cmd")];
  switch (sub) {
    case "services":
      out.push(tok("-listnetworkserviceorder", "sub", "sub"));
      break;
    case "info":
      out.push(tok("-getinfo", "sub", "sub"));
      out.push(service ? tok(quote(service), "value", "service") : tok("<service>", "placeholder", "service"));
      break;
    case "wifi":
      out.push(tok("-getairportpower", "sub", "sub"));
      out.push(device ? tok(device, "value", "device") : tok("<Wi-Fi>", "placeholder", "device"));
      break;
    case "locations":
      out.push(tok("-getcurrentlocation", "sub", "sub"));
      break;
  }
  return out;
}

function quote(value: string): string {
  return value.includes(" ") ? `"${value}"` : value;
}
