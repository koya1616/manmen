import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { DF_FS_TYPES } from "../commands/diskOptions";
import { useDf } from "../hooks/useDf";
import { DfResultView } from "./DfResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, TextField } from "./ui/controls";

export function DfCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useDf();
  const { runner } = form;

  return (
    <Workbench
      active={active}
      title={t("df.title")}
      description={t("df.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("df.execute")}
      runningLabel={t("df.executing")}
      blocker={form.pathOk ? null : t("disk.pathInvalid")}
      about={about}
      options={
        <>
          <div className="group-label">{t("df.groupTarget")}</div>
          <OptionRow
            id="path"
            label={t("df.path")}
            on={form.path.trim() !== ""}
            hint={t("df.hintPath")}
            error={form.pathOk ? null : t("disk.pathInvalid")}
          >
            <TextField
              value={form.path}
              onChange={form.setPath}
              placeholder={t("df.pathPlaceholder")}
              invalid={!form.pathOk}
              onClear={() => form.setPath("")}
            />
          </OptionRow>
          <OptionRow
            id="fsType"
            label={t("df.fsType")}
            flag="-T"
            on={form.fsType !== ""}
            hint={t("df.hintFsType")}
          >
            <Segmented
              value={form.fsType}
              onChange={form.setFsType}
              wrap
              options={[
                { value: "", label: t("df.allTypes") },
                ...DF_FS_TYPES.map((type) => ({ value: type, label: type })),
              ]}
            />
          </OptionRow>
          <FlagRow
            id="local"
            label={t("dfFlags.local")}
            flag="-l"
            hint={t("df.hintLocal")}
            checked={form.local}
            onChange={form.setLocal}
          />
          <FlagRow
            id="all"
            label={t("dfFlags.all")}
            flag="-a"
            hint={t("df.hintAll")}
            checked={form.all}
            onChange={form.setAll}
          />

          <div className="group-label">{t("df.groupDisplay")}</div>
          <FlagRow
            id="hideSystem"
            label={t("dfFlags.hideSystem")}
            flag={t("df.displayOnly")}
            hint={t("df.hintHideSystem")}
            checked={form.hideSystem}
            onChange={form.setHideSystem}
          />
          <p className="note">{t("df.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("df.empty")}
          result={runner.result}
        >
          {runner.result ? (
            <DfResultView result={runner.result} hideSystem={form.hideSystem} />
          ) : null}
        </RunnerOutput>
      }
    />
  );
}
