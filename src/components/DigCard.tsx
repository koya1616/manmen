import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { DIG_CLASSES, DIG_TYPES, type DigTransport, type DigType } from "../commands/digOptions";
import { useDig } from "../hooks/useDig";
import { DigResultView } from "./DigResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, TextField } from "./ui/controls";

const TRANSPORTS: DigTransport[] = ["auto", "4", "6"];

export function DigCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useDig();
  const { runner } = form;

  const blocker = !form.nameOk
    ? t("dig.nameInvalid")
    : !form.serverOk
      ? t("dig.serverInvalid")
      : form.traceConflict
        ? t("dig.traceConflict")
        : null;

  return (
    <Workbench
      active={active}
      title={t("dig.title")}
      description={t("dig.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("dig.execute")}
      runningLabel={t("dig.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("dig.groupQuery")}</div>
          <OptionRow
            id="name"
            label={form.reverse ? t("dig.ip") : t("dig.name")}
            flag={form.reverse ? "-x" : undefined}
            on={form.name.trim() !== ""}
            hint={form.reverse ? t("dig.hintIp") : t("dig.hintName")}
            error={form.nameOk ? null : t("dig.nameInvalid")}
          >
            <TextField
              value={form.name}
              onChange={form.setName}
              placeholder={form.reverse ? t("dig.ipPlaceholder") : t("dig.namePlaceholder")}
              invalid={!form.nameOk}
              onClear={() => form.setName("")}
            />
          </OptionRow>
          {!form.reverse && (
            <>
              <OptionRow id="qtype" label={t("dig.qtype")} on hint={t("dig.hintType")}>
                <Segmented
                  value={form.qtype}
                  onChange={(next) => form.setQtype(next as DigType)}
                  wrap
                  options={DIG_TYPES.map((dtype) => ({ value: dtype, label: dtype }))}
                />
              </OptionRow>
              <OptionRow
                id="qclass"
                label={t("dig.qclass")}
                on={form.qclass !== "IN"}
                hint={t("dig.hintClass")}
              >
                <Segmented
                  value={form.qclass}
                  onChange={form.setQclass}
                  options={DIG_CLASSES.map((qclass) => ({ value: qclass, label: qclass }))}
                />
              </OptionRow>
            </>
          )}
          <OptionRow
            id="server"
            label={t("dig.server")}
            flag="@server"
            on={form.server.trim() !== ""}
            hint={t("dig.hintServer")}
            error={form.serverOk ? null : t("dig.serverInvalid")}
          >
            <TextField
              value={form.server}
              onChange={form.setServer}
              placeholder={t("dig.serverPlaceholder")}
              invalid={!form.serverOk}
              onClear={() => form.setServer("")}
            />
          </OptionRow>

          <div className="group-label">{t("dig.groupDisplay")}</div>
          <FlagRow
            id="short"
            label={t("digFlags.short")}
            flag="+short"
            hint={t("dig.hintShort")}
            checked={form.short}
            onChange={form.setShort}
          />
          <FlagRow
            id="tcp"
            label={t("digFlags.tcp")}
            flag="+tcp"
            hint={t("dig.hintTcp")}
            checked={form.tcp}
            onChange={form.setTcp}
          />
          <FlagRow
            id="dnssec"
            label={t("digFlags.dnssec")}
            flag="+dnssec"
            hint={t("dig.hintDnssec")}
            checked={form.dnssec}
            onChange={form.setDnssec}
          />
          <FlagRow
            id="trace"
            label={t("digFlags.trace")}
            flag="+trace"
            hint={t("dig.hintTrace")}
            checked={form.trace}
            onChange={form.setTrace}
          />
          <FlagRow
            id="noRecurse"
            label={t("digFlags.noRecurse")}
            flag="+norecurse"
            hint={t("dig.hintNoRecurse")}
            checked={form.noRecurse}
            onChange={form.setNoRecurse}
          />
          <FlagRow
            id="reverse"
            label={t("digFlags.reverse")}
            flag="-x"
            hint={t("dig.hintReverse")}
            checked={form.reverse}
            onChange={form.setReverse}
          />
          <OptionRow
            id="transport"
            label={t("dig.transport")}
            flag="-4 / -6"
            on={form.transport !== "auto"}
            hint={t("dig.hintTransport")}
          >
            <Segmented
              value={form.transport}
              onChange={form.setTransport}
              options={TRANSPORTS.map((mode) => ({
                value: mode,
                label: t(`digTransport.${mode}`),
              }))}
            />
          </OptionRow>
          <p className="note">{t("dig.note")}</p>
        </>
      }
      output={
        <OutputPane
          running={runner.running}
          error={runner.error}
          meta={
            runner.result
              ? {
                  success: runner.result.success,
                  exitCode: runner.result.exit_code,
                  stderr: runner.result.stderr,
                }
              : null
          }
          ranAt={runner.ranAt}
          durationMs={runner.durationMs}
          ranCommand={runner.ranCommand}
          stale={runner.ranCommand !== null && runner.ranCommand !== form.preview}
          emptyHint={t("dig.empty")}
        >
          {runner.result ? <DigResultView result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
