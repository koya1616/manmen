import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  HIBERNATE_VALUES,
  MAX_PMSET_MINUTES,
  PMSET_SCOPES,
  PMSET_SETTINGS,
  type PmsetScope,
  type PmsetValueKind,
} from "../commands/pmsetSettings";
import { currentValue, usePmset } from "../hooks/usePmset";
import type { PmsetState } from "../types";
import { CommandResultView } from "./CommandResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { useLink, Workbench } from "./ui/Workbench";
import { OptionRow, Segmented, Stepper, Toggle } from "./ui/controls";

function formatScalar(
  kind: PmsetValueKind,
  value: string,
  t: (key: string) => string,
): string {
  if (kind === "bool") return value === "1" ? "ON (1)" : "OFF (0)";
  if (kind === "minutes") return `${value} ${t("pmset.minutesUnit")}`;
  return value;
}

function currentLabel(
  state: PmsetState | null,
  setting: string,
  scope: PmsetScope,
  kind: PmsetValueKind,
  t: (key: string) => string,
): string {
  if (setting === "disablesleep" || scope !== "a") {
    const value = currentValue(state, setting, scope === "a" ? "a" : scope);
    if (!value) return t("pmset.unknown");
    return formatScalar(kind, value, t);
  }
  const parts = (
    [
      ["b", state?.battery],
      ["c", state?.ac],
      ["u", state?.ups],
    ] as const
  )
    .map(([source, values]) => {
      const found = values?.find((item) => item.name === setting)?.value;
      if (!found) return null;
      return `${t(`pmsetScopeShort.${source}`)} ${formatScalar(kind, found, t)}`;
    })
    .filter((part): part is string => part !== null);
  return parts.length > 0 ? parts.join(" / ") : t("pmset.unknown");
}

const MINUTE_PRESETS = ["0", "1", "5", "10", "15", "30", "60", "180"];

// 設定項目の一覧。各行に現在値を出し、選ぶと値の欄がその種類に合わせて切り替わる。
function SettingList({
  setting,
  scope,
  state,
  onPick,
}: {
  setting: string;
  scope: PmsetScope;
  state: PmsetState | null;
  onPick: (name: string) => void;
}) {
  const { t } = useTranslation();
  const { linked, flash, setLinked } = useLink();

  return (
    <section
      data-opt="setting"
      className={`opt is-on ${linked === "setting" ? "is-linked" : ""} ${flash === "setting" ? "is-flash" : ""}`}
      onMouseEnter={() => setLinked("setting")}
      onMouseLeave={() => setLinked(null)}
    >
      <div className="opt-head">
        <span className="opt-label">{t("pmset.setting")}</span>
        <code className="opt-flag">{setting}</code>
      </div>
      <div className="setting-list">
        {PMSET_SETTINGS.map((item) => (
          <button
            key={item.name}
            type="button"
            className={`setting-item ${item.name === setting ? "is-selected" : ""}`}
            onClick={() => onPick(item.name)}
          >
            <span className="setting-copy">
              <span className="setting-label">{t(`pmsetSettings.${item.name}`)}</span>
              <code>{item.name}</code>
            </span>
            <span className="setting-current">
              {currentLabel(state, item.name, scope, item.kind, t)}
            </span>
          </button>
        ))}
      </div>
    </section>
  );
}

export function PmsetCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const {
    scope,
    setting,
    value,
    kind,
    state,
    remember,
    runner,
    valid,
    tokens,
    preview,
    chooseScope,
    chooseSetting,
    chooseValue,
    execute,
    updateRemember,
  } = usePmset();

  const current = currentLabel(state, setting, scope, kind, t);
  const next = valid ? formatScalar(kind, kind === "minutes" ? String(Number(value)) : value, t) : "…";

  return (
    <Workbench
      active={active}
      title={t("pmset.title")}
      description={t("pmset.description")}
      tokens={tokens}
      onRun={execute}
      canRun={valid}
      running={runner.running}
      runLabel={t("pmset.execute")}
      runningLabel={t("pmset.executing")}
      blocker={valid ? null : t("pmset.minutesHint")}
      notice={
        <span>
          🔒 {t("pmset.adminNote")}
        </span>
      }
      about={about}
      options={
        <>
          <OptionRow id="scope" label={t("pmset.scope")} flag={`-${scope}`} on>
            <Segmented
              value={scope}
              onChange={chooseScope}
              wrap
              options={PMSET_SCOPES.map((item) => ({
                value: item,
                label: t(`pmsetScopeShort.${item}`),
                sub: `-${item}`,
              }))}
            />
          </OptionRow>

          <OptionRow
            id="value"
            label={t("pmset.value")}
            flag={valid ? value : "…"}
            on
            hint={
              <span className="diff">
                <span>{t("pmset.current")}: {current}</span>
                <span aria-hidden>→</span>
                <strong>{next}</strong>
              </span>
            }
          >
            {kind === "bool" ? (
              <Toggle
                checked={value === "1"}
                onChange={(on) => chooseValue(on ? "1" : "0")}
                onLabel="ON (1)"
                offLabel="OFF (0)"
              />
            ) : null}
            {kind === "minutes" ? (
              <Stepper
                value={value}
                onChange={chooseValue}
                min={0}
                max={MAX_PMSET_MINUTES}
                unit={t("pmset.minutesUnit")}
                presets={MINUTE_PRESETS.map((v) => ({
                  value: v,
                  label: v === "0" ? t("pmset.never") : `${v}${t("pmset.minutesUnit")}`,
                }))}
              />
            ) : null}
            {kind === "hibernate" ? (
              <Segmented
                value={value}
                onChange={chooseValue}
                options={HIBERNATE_VALUES.map((item) => ({
                  value: item,
                  label: t(`pmsetHibernate.${item}`),
                  sub: item,
                }))}
              />
            ) : null}
          </OptionRow>

          <SettingList setting={setting} scope={scope} state={state} onPick={chooseSetting} />

          <OptionRow id="remember" label={t("pmset.remember")} hint={t("pmset.rememberNote")}>
            <Toggle checked={remember} onChange={updateRemember} />
          </OptionRow>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("pmset.empty")}
          result={runner.result}
        >
          {runner.result ? <CommandResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
