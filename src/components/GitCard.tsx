import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { GIT_SUBS, useGit } from "../hooks/useGit";
import { GitBranchesResultView } from "./GitBranchesResultView";
import { GitLogResultView } from "./GitLogResultView";
import { GitRemotesResultView } from "./GitRemotesResultView";
import { GitStatusResultView } from "./GitStatusResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { OptionRow, Segmented, TextField } from "./ui/controls";

export function GitCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { sub, setSub, dir, setDir, suggestions, runner, tokens, preview, valid, execute } =
    useGit();
  const out = runner.result;

  return (
    <Workbench
      active={active}
      title={t("git.title")}
      description={t("git.description")}
      tokens={tokens}
      onRun={execute}
      canRun={valid}
      running={runner.running}
      runLabel={t("git.execute")}
      runningLabel={t("git.executing")}
      blocker={valid ? null : t("git.dirRequired")}
      about={about}
      options={
        <>
          <OptionRow
            id="dir"
            label={t("git.dir")}
            flag={dir.trim() || "…"}
            on={valid}
            hint={t("git.hintDir")}
          >
            <TextField
              value={dir}
              onChange={setDir}
              placeholder={t("git.dirPlaceholder")}
              suggestions={suggestions}
              onClear={() => setDir("")}
            />
          </OptionRow>
          <OptionRow id="sub" label={t("git.sub")} flag={sub} on>
            <Segmented
              value={sub}
              onChange={setSub}
              wrap
              options={GIT_SUBS.map((name) => ({
                value: name,
                label: t(`gitSub.${name}`),
                sub: name,
              }))}
            />
            <p className="opt-hint">{t(`gitSubHelp.${sub}`)}</p>
          </OptionRow>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={preview}
          emptyHint={t("git.empty")}
          result={out}
        >
          {out?.kind === "status" ? <GitStatusResultView result={out} /> : null}
          {out?.kind === "log" ? <GitLogResultView result={out} /> : null}
          {out?.kind === "branches" ? <GitBranchesResultView result={out} /> : null}
          {out?.kind === "remotes" ? <GitRemotesResultView result={out} /> : null}
        </RunnerOutput>
      }
    />
  );
}
