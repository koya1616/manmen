import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useWho } from "../hooks/useWho";
import { WhoResultView } from "./WhoResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";

export function WhoCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { runner, tokens, preview, execute } = useWho();

  return (
    <Workbench
      active={active}
      title={t("who.title")}
      description={t("who.description")}
      tokens={tokens}
      onRun={execute}
      canRun
      running={runner.running}
      runLabel={t("who.execute")}
      runningLabel={t("who.executing")}
      about={about}
      options={<p className="muted">{t("who.noOptions")}</p>}
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("who.empty")}
          result={runner.result}
        >
          {runner.result ? <WhoResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
