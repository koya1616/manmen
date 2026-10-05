import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useW } from "../hooks/useW";
import { WResultView } from "./WResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";

export function WCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { runner, tokens, preview, execute } = useW();

  return (
    <Workbench
      active={active}
      title={t("w.title")}
      description={t("w.description")}
      tokens={tokens}
      onRun={execute}
      canRun
      running={runner.running}
      runLabel={t("w.execute")}
      runningLabel={t("w.executing")}
      about={about}
      options={<p className="muted">{t("w.noOptions")}</p>}
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("w.empty")}
          result={runner.result}
        >
          {runner.result ? <WResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
