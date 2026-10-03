import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useW } from "../hooks/useW";
import { WResultView } from "./WResultView";
import { OutputPane } from "./ui/OutputPane";
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
          emptyHint={t("w.empty")}
        >
          {runner.result ? <WResultView result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
