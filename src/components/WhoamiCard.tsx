import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useWhoami } from "../hooks/useWhoami";
import { CommandResultView } from "./CommandResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";

export function WhoamiCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { runner, tokens, preview, execute } = useWhoami();

  return (
    <Workbench
      active={active}
      title={t("whoami.title")}
      description={t("whoami.description")}
      tokens={tokens}
      onRun={execute}
      canRun
      running={runner.running}
      runLabel={t("whoami.execute")}
      runningLabel={t("whoami.executing")}
      about={about}
      options={<p className="muted">{t("whoami.noOptions")}</p>}
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("whoami.empty")}
          result={runner.result}
        >
          {runner.result ? <CommandResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
