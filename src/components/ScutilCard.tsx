import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { SCUTIL_SUBS, useScutil } from "../hooks/useScutil";
import { ScutilDnsResultView } from "./ScutilDnsResultView";
import { ScutilNwiResultView } from "./ScutilNwiResultView";
import { ScutilNamesResultView } from "./ScutilNamesResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import { OptionRow, Segmented } from "./ui/controls";

export function ScutilCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { sub, setSub, runner, tokens, preview, execute } = useScutil();
  const out = runner.result;

  return (
    <Workbench
      active={active}
      title={t("scutil.title")}
      description={t("scutil.description")}
      tokens={tokens}
      onRun={execute}
      canRun
      running={runner.running}
      runLabel={t("scutil.execute")}
      runningLabel={t("scutil.executing")}
      about={about}
      options={
        <OptionRow id="sub" label={t("scutil.sub")} flag={sub} on>
          <Segmented
            value={sub}
            onChange={setSub}
            wrap
            options={SCUTIL_SUBS.map((name) => ({
              value: name,
              label: t(`scutilSub.${name}`),
              sub: name,
            }))}
          />
          <p className="opt-hint">{t(`scutilSubHelp.${sub}`)}</p>
        </OptionRow>
      }
      output={
        <OutputPane
          running={runner.running}
          error={runner.error}
          meta={
            out
              ? {
                  success: out.success,
                  exitCode: out.exit_code,
                  stderr: out.stderr,
                }
              : null
          }
          ranAt={runner.ranAt}
          durationMs={runner.durationMs}
          ranCommand={runner.ranCommand}
          stale={runner.ranCommand !== null && runner.ranCommand !== preview}
          emptyHint={t("scutil.empty")}
        >
          {out?.kind === "dns" ? <ScutilDnsResultView result={out} /> : null}
          {out?.kind === "nwi" ? <ScutilNwiResultView result={out} /> : null}
          {out?.kind === "names" ? <ScutilNamesResultView result={out} /> : null}
        </OutputPane>
      }
    />
  );
}
