import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { SYSTEM_PROFILER_TYPES } from "../commands/systemProfilerTypes";
import { useSystemProfiler } from "../hooks/useSystemProfiler";
import { SystemProfilerResultView } from "./SystemProfilerResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { OptionRow, Segmented } from "./ui/controls";

export function SystemProfilerCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { dataType, setDataType, runner, tokens, preview, valid, execute } =
    useSystemProfiler();

  return (
    <Workbench
      active={active}
      title={t("sp.title")}
      description={t("sp.description")}
      tokens={tokens}
      onRun={execute}
      canRun={valid}
      running={runner.running}
      runLabel={t("sp.execute")}
      runningLabel={t("sp.executing")}
      about={about}
      options={
        <>
          <OptionRow id="dataType" label={t("sp.dataType")} flag={dataType} on>
            <Segmented
              value={dataType}
              onChange={setDataType}
              wrap
              options={SYSTEM_PROFILER_TYPES.map((name) => ({
                value: name,
                label: t(`spTypes.${name}`),
                sub: name.replace(/^SP|DataType$/g, ""),
              }))}
            />
            <p className="opt-hint">{t(`spTypeHelp.${dataType}`)}</p>
          </OptionRow>
          <p className="note">{t("sp.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("sp.empty")}
          result={runner.result}
        >
          {runner.result && runner.result.value !== null ? (
            <SystemProfilerResultView
              key={runner.ranCommand}
              value={runner.result.value}
            />
          ) : null}
        </RunnerOutput>
      }
    />
  );
}
