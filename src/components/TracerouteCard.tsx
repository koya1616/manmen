import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  TRACEROUTE_MAX_MAX_TTL,
  TRACEROUTE_MAX_QUERIES,
  TRACEROUTE_MAX_WAIT,
  TRACEROUTE_MAX_WORST_SECS,
} from "../commands/tracerouteOptions";
import { useTraceroute } from "../hooks/useTraceroute";
import { TracerouteResultView } from "./TracerouteResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, Stepper, TextField } from "./ui/controls";

export function TracerouteCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useTraceroute();
  const { runner } = form;

  const worstMessage = t("traceroute.worstInvalid", {
    secs: form.worstSecs,
    max: TRACEROUTE_MAX_WORST_SECS,
  });
  const blocker = !form.hostOk
    ? t("ping.hostInvalid")
    : !form.maxTtlOk
      ? t("traceroute.maxTtlInvalid")
      : !form.firstTtlOk
        ? t("traceroute.firstTtlInvalid")
        : !form.queriesOk
          ? t("traceroute.queriesInvalid")
          : !form.waitOk
            ? t("traceroute.waitInvalid")
            : !form.worstOk
              ? worstMessage
              : null;

  return (
    <Workbench
      active={active}
      title={t("traceroute.title")}
      description={t("traceroute.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("traceroute.execute")}
      runningLabel={t("traceroute.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("traceroute.groupTarget")}</div>
          <OptionRow
            id="host"
            label={t("traceroute.host")}
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
            id="protocol"
            label={t("traceroute.protocol")}
            on={form.icmp}
            hint={t("traceroute.hintProtocol")}
          >
            <Segmented
              value={form.icmp ? "icmp" : "udp"}
              onChange={(next) => form.setIcmp(next === "icmp")}
              options={[
                { value: "udp", label: "UDP", sub: t("traceroute.default") },
                { value: "icmp", label: "ICMP", sub: "-I" },
              ]}
            />
          </OptionRow>

          <div className="group-label">{t("traceroute.groupRange")}</div>
          <OptionRow
            id="maxTtl"
            label={t("traceroute.maxTtl")}
            flag="-m"
            on
            hint={t("traceroute.hintMaxTtl")}
            error={form.maxTtlOk ? null : t("traceroute.maxTtlInvalid")}
          >
            <Stepper
              value={form.maxTtl}
              onChange={form.setMaxTtl}
              min={1}
              max={TRACEROUTE_MAX_MAX_TTL}
              step={1}
            />
          </OptionRow>
          <OptionRow
            id="firstTtl"
            label={t("traceroute.firstTtl")}
            flag="-f"
            on={form.firstTtl.trim() !== ""}
            hint={t("traceroute.hintFirstTtl")}
            error={form.firstTtlOk ? null : t("traceroute.firstTtlInvalid")}
          >
            <TextField
              value={form.firstTtl}
              onChange={form.setFirstTtl}
              placeholder={t("traceroute.firstTtlPlaceholder")}
              invalid={!form.firstTtlOk}
              onClear={() => form.setFirstTtl("")}
            />
          </OptionRow>

          <div className="group-label">{t("traceroute.groupTiming")}</div>
          <OptionRow
            id="queries"
            label={t("traceroute.queries")}
            flag="-q"
            on
            hint={t("traceroute.hintQueries")}
            error={form.queriesOk ? null : t("traceroute.queriesInvalid")}
          >
            <Stepper
              value={form.queries}
              onChange={form.setQueries}
              min={1}
              max={TRACEROUTE_MAX_QUERIES}
              step={1}
            />
          </OptionRow>
          <OptionRow
            id="wait"
            label={t("traceroute.wait")}
            flag="-w"
            on
            hint={t("traceroute.hintWait")}
            error={form.waitOk ? (form.worstOk ? null : worstMessage) : t("traceroute.waitInvalid")}
          >
            <Stepper
              value={form.wait}
              onChange={form.setWait}
              min={1}
              max={TRACEROUTE_MAX_WAIT}
              step={1}
              unit={t("curl.seconds")}
            />
          </OptionRow>
          <p className="note">
            {t("traceroute.worst", { secs: form.worstSecs, max: TRACEROUTE_MAX_WORST_SECS })}
          </p>

          <div className="group-label">{t("traceroute.groupDisplay")}</div>
          <FlagRow
            id="numeric"
            label={t("tracerouteFlags.numeric")}
            flag="-n"
            hint={t("traceroute.hintNumeric")}
            checked={form.numeric}
            onChange={form.setNumeric}
          />
          <FlagRow
            id="asLookup"
            label={t("tracerouteFlags.asLookup")}
            flag="-a"
            hint={t("traceroute.hintAsLookup")}
            checked={form.asLookup}
            onChange={form.setAsLookup}
          />
          <p className="note">{t("traceroute.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("traceroute.empty")}
          result={runner.result}
        >
          {runner.result ? <TracerouteResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
