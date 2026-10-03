import { useCallback, useEffect, useState } from "react";
import { api } from "../api";
import type { SshKnownHost } from "../types";

// `~/.ssh/config` の既知ホストを裏で読む。失敗しても空のまま動く。
export function useSshHosts(active: boolean) {
  const [hosts, setHosts] = useState<SshKnownHost[]>([]);

  const reload = useCallback(async () => {
    try {
      setHosts(await api.listSshHosts());
    } catch {
      // 設定が無い・読めない場合は候補なしで続ける
    }
  }, []);

  useEffect(() => {
    if (active) reload();
  }, [active, reload]);

  return { hosts, reload };
}
