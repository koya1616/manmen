import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useWhoami } from "../hooks/useWhoami";
import { CommandResultView } from "./CommandResultView";
import { OutputPane } from "./ui/OutputPane";
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
          emptyHint={t("whoami.empty")}
        >
          {runner.result ? <CommandResultView result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
