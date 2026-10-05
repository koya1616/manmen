import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  PING_MAX_COUNT,
  PING_MAX_SIZE,
} from "../commands/pingOptions";
import { usePing } from "../hooks/usePing";
import { PingResultView } from "./PingResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Stepper, TextField } from "./ui/controls";

export function PingCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = usePing();
  const { runner } = form;

  const blocker = !form.hostOk
    ? t("ping.hostInvalid")
    : !form.countOk
      ? t("ping.countInvalid")
      : !form.intervalOk
        ? t("ping.intervalInvalid")
        : !form.timeoutOk
          ? t("ping.timeoutInvalid")
          : !form.waitOk
            ? t("ping.waitInvalid")
            : !form.sizeOk
              ? t("ping.sizeInvalid")
              : !form.ttlOk
                ? t("ping.ttlInvalid")
                : null;

  return (
    <Workbench
      active={active}
      title={t("ping.title")}
      description={t("ping.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("ping.execute")}
      runningLabel={t("ping.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("ping.groupTarget")}</div>
          <OptionRow
            id="host"
            label={t("ping.host")}
            on={form.host.trim() !== ""}
            hint={t("ping.hintHost")}
            error={form.hostOk ? null : t("ping.hostInvalid")}
          >
            <TextField
              value={form.host}
              onChange={form.setHost}
              placeholder={t("ping.hostPlaceholder")}
              invalid={!form.hostOk}
              onClear={() => form.setHost("")}
            />
          </OptionRow>
          <OptionRow
            id="count"
            label={t("ping.count")}
            flag="-c"
            on
            hint={t("ping.hintCount")}
            error={form.countOk ? null : t("ping.countInvalid")}
          >
            <Stepper
              value={form.count}
              onChange={form.setCount}
              min={1}
              max={PING_MAX_COUNT}
              step={1}
            />
          </OptionRow>

          <div className="group-label">{t("ping.groupTiming")}</div>
          <OptionRow
            id="interval"
            label={t("ping.interval")}
            flag="-i"
            on={form.interval.trim() !== "1"}
            hint={t("ping.hintInterval")}
            error={form.intervalOk ? null : t("ping.intervalInvalid")}
          >
            <TextField
              value={form.interval}
              onChange={form.setInterval}
              placeholder="1"
              invalid={!form.intervalOk}
              onClear={() => form.setInterval("1")}
            />
          </OptionRow>
          <OptionRow
            id="timeout"
            label={t("ping.timeout")}
            flag="-t"
            on={form.timeout.trim() !== ""}
            hint={t("ping.hintTimeout")}
            error={form.timeoutOk ? null : t("ping.timeoutInvalid")}
          >
            <TextField
              value={form.timeout}
              onChange={form.setTimeout}
              placeholder={t("ping.timeoutPlaceholder")}
              invalid={!form.timeoutOk}
              onClear={() => form.setTimeout("")}
            />
          </OptionRow>
          <OptionRow
            id="waitMs"
            label={t("ping.waitMs")}
            flag="-W"
            on={form.waitMs.trim() !== ""}
            hint={t("ping.hintWaitMs")}
            error={form.waitOk ? null : t("ping.waitInvalid")}
          >
            <TextField
              value={form.waitMs}
              onChange={form.setWaitMs}
              placeholder={t("ping.waitPlaceholder")}
              invalid={!form.waitOk}
              onClear={() => form.setWaitMs("")}
            />
          </OptionRow>

          <div className="group-label">{t("ping.groupPacket")}</div>
          <OptionRow
            id="size"
            label={t("ping.size")}
            flag="-s"
            on={form.size.trim() !== "56"}
            hint={t("ping.hintSize")}
            error={form.sizeOk ? null : t("ping.sizeInvalid")}
          >
            <Stepper
              value={form.size}
              onChange={form.setSize}
              min={0}
              max={PING_MAX_SIZE}
              step={8}
            />
          </OptionRow>
          <OptionRow
            id="ttl"
            label={t("ping.ttl")}
            flag="-m"
            on={form.ttl.trim() !== ""}
            hint={t("ping.hintTtl")}
            error={form.ttlOk ? null : t("ping.ttlInvalid")}
          >
            <TextField
              value={form.ttl}
              onChange={form.setTtl}
              placeholder={t("ping.ttlPlaceholder")}
              invalid={!form.ttlOk}
              onClear={() => form.setTtl("")}
            />
          </OptionRow>
          <FlagRow
            id="numeric"
            label={t("pingFlags.numeric")}
            flag="-n"
            hint={t("ping.hintNumeric")}
            checked={form.numeric}
            onChange={form.setNumeric}
          />
          <FlagRow
            id="noFragment"
            label={t("pingFlags.noFragment")}
            flag="-D"
            hint={t("ping.hintNoFragment")}
            checked={form.noFragment}
            onChange={form.setNoFragment}
          />
          <p className="note">{t("ping.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("ping.empty")}
          result={runner.result}
        >
          {runner.result ? <PingResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
