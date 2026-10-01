// Rust の pmset.rs SETTINGS / SCOPES と一致させること。

export const PMSET_SCOPES = ["a", "b", "c", "u"] as const;

export type PmsetScope = (typeof PMSET_SCOPES)[number];

export type PmsetValueKind = "bool" | "minutes" | "hibernate";

export interface PmsetSettingDef {
  name: string;
  kind: PmsetValueKind;
}

export const PMSET_SETTINGS: PmsetSettingDef[] = [
  { name: "disablesleep", kind: "bool" },
  { name: "displaysleep", kind: "minutes" },
  { name: "disksleep", kind: "minutes" },
  { name: "sleep", kind: "minutes" },
  { name: "womp", kind: "bool" },
  { name: "ring", kind: "bool" },
  { name: "powernap", kind: "bool" },
  { name: "proximitywake", kind: "bool" },
  { name: "autorestart", kind: "bool" },
  { name: "autorestartatconnect", kind: "bool" },
  { name: "lidwake", kind: "bool" },
  { name: "acwake", kind: "bool" },
  { name: "lessbright", kind: "bool" },
  { name: "halfdim", kind: "bool" },
  { name: "sms", kind: "bool" },
  { name: "ttyskeepawake", kind: "bool" },
  { name: "destroyfvkeyonstandby", kind: "bool" },
  { name: "hibernatemode", kind: "hibernate" },
];

export const HIBERNATE_VALUES = ["0", "3", "25"] as const;

export const DEFAULT_PMSET_SCOPE: PmsetScope = "a";
export const DEFAULT_PMSET_SETTING = "disablesleep";

export function pmsetSetting(name: string): PmsetSettingDef {
  return (
    PMSET_SETTINGS.find((item) => item.name === name) ?? PMSET_SETTINGS[0]
  );
}

export function defaultValue(kind: PmsetValueKind): string {
  if (kind === "hibernate") return "3";
  return "0";
}

export const MAX_PMSET_MINUTES = 1440;

export function isValidPmsetValue(kind: PmsetValueKind, value: string): boolean {
  if (kind === "bool") return value === "0" || value === "1";
  if (kind === "hibernate") return (HIBERNATE_VALUES as readonly string[]).includes(value);
  if (!/^\d+$/.test(value)) return false;
  const minutes = Number(value);
  return minutes >= 0 && minutes <= MAX_PMSET_MINUTES;
}
