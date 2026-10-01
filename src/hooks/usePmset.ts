import { useEffect, useRef, useState } from "react";
import { api } from "../api";
import {
  DEFAULT_PMSET_SCOPE,
  DEFAULT_PMSET_SETTING,
  defaultValue,
  isValidPmsetValue,
  pmsetSetting,
  type PmsetScope,
} from "../commands/pmsetSettings";
import type { CommandResult, PmsetState, PmsetValue } from "../types";

function findValue(values: PmsetValue[], name: string): string | null {
  return values.find((item) => item.name === name)?.value ?? null;
}

export function currentValue(
  state: PmsetState | null,
  setting: string,
  scope: PmsetScope,
): string | null {
  if (!state) return null;
  if (setting === "disablesleep") return state.sleep_disabled;
  if (scope === "b") return findValue(state.battery, setting);
  if (scope === "c") return findValue(state.ac, setting);
  if (scope === "u") return findValue(state.ups, setting);
  return null;
}

function seedValue(
  state: PmsetState | null,
  setting: string,
  scope: PmsetScope,
): string {
  const kind = pmsetSetting(setting).kind;
  const candidates =
    setting === "disablesleep"
      ? [state?.sleep_disabled ?? null]
      : scope === "a"
        ? [
            findValue(state?.battery ?? [], setting),
            findValue(state?.ac ?? [], setting),
            findValue(state?.ups ?? [], setting),
          ]
        : [currentValue(state, setting, scope)];
  return (
    candidates.find((item) => item !== null && isValidPmsetValue(kind, item)) ??
    defaultValue(kind)
  );
}

export function usePmset() {
  const [scope, setScope] = useState<PmsetScope>(DEFAULT_PMSET_SCOPE);
  const [setting, setSetting] = useState(DEFAULT_PMSET_SETTING);
  const [value, setValue] = useState("0");
  const [state, setState] = useState<PmsetState | null>(null);
  const [remember, setRemember] = useState(true);
  const [executing, setExecuting] = useState(false);
  const [result, setResult] = useState<CommandResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const edited = useRef(false);

  const kind = pmsetSetting(setting).kind;
  const valid = isValidPmsetValue(kind, value);
  const previewValue =
    kind === "minutes" && valid ? String(Number(value)) : value;
  const preview = `sudo pmset -${scope} ${setting} ${valid ? previewValue : "..."}`;

  async function refresh() {
    const next = await api.getPmset();
    setState(next);
    return next;
  }

  useEffect(() => {
    refresh()
      .then((next) => {
        if (edited.current) return;
        setValue(seedValue(next, DEFAULT_PMSET_SETTING, DEFAULT_PMSET_SCOPE));
      })
      .catch(() => {
        // pmset -g が読めない環境では現在値不明のままにする
      });
  }, []);

  function chooseScope(next: PmsetScope) {
    edited.current = true;
    setScope(next);
    setValue(seedValue(state, setting, next));
  }

  function chooseSetting(next: string) {
    edited.current = true;
    setSetting(next);
    setValue(seedValue(state, next, scope));
  }

  function chooseValue(next: string) {
    edited.current = true;
    setValue(next);
  }

  async function execute() {
    if (!valid) return;
    setExecuting(true);
    setResult(null);
    setError(null);
    try {
      const res = await api.setPmset(scope, setting, previewValue);
      setResult(res);
      if (res.success) {
        await refresh().catch(() => undefined);
      }
    } catch (e) {
      setError(String(e));
    } finally {
      setExecuting(false);
    }
  }

  async function updateRemember(next: boolean) {
    setRemember(next);
    await api.setRemember(next);
  }

  return {
    scope,
    setting,
    value,
    kind,
    state,
    remember,
    executing,
    result,
    error,
    valid,
    preview,
    chooseScope,
    chooseSetting,
    chooseValue,
    execute,
    updateRemember,
  };
}
