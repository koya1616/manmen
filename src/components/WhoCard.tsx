import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useWho } from "../hooks/useWho";
import { WhoResultView } from "./WhoResultView";
import { OutputPane } from "./ui/OutputPane";
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
          stale={runner.ranCommand !== null && runner.ranCommand !== preview}
          emptyHint={t("who.empty")}
        >
          {runner.result ? <WhoResultView result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
