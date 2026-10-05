import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { DU_MAX_DEPTH, DU_MAX_TIMEOUT_SECS, DU_MIN_TIMEOUT_SECS } from "../commands/diskOptions";
import { useDu } from "../hooks/useDu";
import { DuResultView } from "./DuResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Stepper, TextField } from "./ui/controls";

export function DuCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useDu();
  const { runner } = form;

  const blocker = !form.pathOk
    ? t("disk.pathInvalid")
    : !form.depthOk
      ? t("du.depthInvalid")
      : !form.timeoutOk
        ? t("du.timeoutInvalid")
        : null;

  return (
    <Workbench
      active={active}
      title={t("du.title")}
      description={t("du.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("du.execute")}
      runningLabel={t("du.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("du.groupTarget")}</div>
          <OptionRow
            id="path"
            label={t("du.path")}
            on={form.path.trim() !== ""}
            hint={t("du.hintPath")}
            error={form.pathOk ? null : t("disk.pathInvalid")}
          >
            <TextField
              value={form.path}
              onChange={form.setPath}
              placeholder={t("du.pathPlaceholder")}
              invalid={!form.pathOk}
              onClear={() => form.setPath("")}
              suggestions={["~", "~/Library", "~/Downloads", "/Applications"]}
            />
          </OptionRow>
          <OptionRow
            id="depth"
            label={t("du.depth")}
            flag="-d"
            on
            hint={t("du.hintDepth")}
            error={form.depthOk ? null : t("du.depthInvalid")}
          >
            <Stepper value={form.depth} onChange={form.setDepth} min={0} max={DU_MAX_DEPTH} step={1} />
          </OptionRow>
          <FlagRow
            id="allFiles"
            label={t("duFlags.allFiles")}
            flag="-a"
            hint={t("du.hintAllFiles")}
            checked={form.allFiles}
            onChange={form.setAllFiles}
          />

          <div className="group-label">{t("du.groupCount")}</div>
          <FlagRow
            id="oneFs"
            label={t("duFlags.oneFs")}
            flag="-x"
            hint={t("du.hintOneFs")}
            checked={form.oneFs}
            onChange={form.setOneFs}
          />
          <FlagRow
            id="apparent"
            label={t("duFlags.apparent")}
            flag="-A"
            hint={t("du.hintApparent")}
            checked={form.apparent}
            onChange={form.setApparent}
          />

          <div className="group-label">{t("du.groupTiming")}</div>
          <OptionRow
            id="timeout"
            label={t("du.timeout")}
            on
            hint={t("du.hintTimeout")}
            error={form.timeoutOk ? null : t("du.timeoutInvalid")}
          >
            <Stepper
              value={form.timeoutSecs}
              onChange={form.setTimeoutSecs}
              min={DU_MIN_TIMEOUT_SECS}
              max={DU_MAX_TIMEOUT_SECS}
              step={5}
              unit={t("curl.seconds")}
            />
          </OptionRow>
          <p className="note">{t("du.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("du.empty")}
          result={runner.result}
        >
          {runner.result ? <DuResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
