import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { DOCKER_SUBS, useDocker } from "../hooks/useDocker";
import { CommandResultView } from "./CommandResultView";
import { DockerDuResultView } from "./DockerDuResultView";
import { DockerContainersResultView } from "./DockerContainersResultView";
import { DockerImagesResultView } from "./DockerImagesResultView";
import { DockerNetworksResultView } from "./DockerNetworksResultView";
import { DockerVolumesResultView } from "./DockerVolumesResultView";
import { DockerSystemDfResultView } from "./DockerSystemDfResultView";
import { DockerSystemInfoResultView } from "./DockerSystemInfoResultView";
import { DockerInspectResultView } from "./DockerInspectResultView";
import { DockerLsResultView } from "./DockerLsResultView";
import { DockerVersionResultView } from "./DockerVersionResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, TextField } from "./ui/controls";

export function DockerCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useDocker();
  const { runner } = form;
  const danger = form.sub === "prune";
  const out = runner.result;

  return (
    <Workbench
      active={active}
      title={t("docker.title")}
      description={t("docker.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={danger ? t("docker.execute") : t("docker.run")}
      runningLabel={danger ? t("docker.executing") : t("docker.readExecuting")}
      danger={danger}
      blocker={form.valid ? null : t("docker.nameInvalid")}
      notice={danger ? <span>⚠️ {t("docker.note")}</span> : null}
      about={about}
      options={
        <>
          <OptionRow id="sub" label={t("docker.sub")} flag={form.sub} on>
            <Segmented
              value={form.sub}
              onChange={form.setSub}
              wrap
              options={DOCKER_SUBS.map((sub) => ({
                value: sub,
                label: t(`dockerSub.${sub}`),
                sub,
              }))}
            />
            <p className="opt-hint">{t(`dockerSubHelp.${form.sub}`)}</p>
          </OptionRow>

          {form.sub === "inspect" ? (
            <OptionRow
              id="name"
              label={t("docker.name")}
              on={form.name.trim() !== ""}
              hint={t("docker.hintName")}
              error={form.nameOk ? null : t("docker.nameInvalid")}
            >
              <TextField
                value={form.name}
                onChange={form.setName}
                placeholder={t("docker.namePlaceholder")}
                invalid={!form.nameOk}
                onClear={() => form.setName("")}
              />
            </OptionRow>
          ) : null}

          {form.sub === "prune" ? (
            <>
              <FlagRow
                id="force"
                label={t("dockerFlags.force")}
                flag="-f"
                hint={t("docker.hintForce")}
                checked={form.force}
                onChange={form.setForce}
              />
              <FlagRow
                id="all"
                label={t("dockerFlags.all")}
                flag="--all"
                hint={t("docker.hintAll")}
                checked={form.all}
                onChange={form.setAll}
              />
            </>
          ) : null}

          {form.sub !== "inspect" && form.sub !== "prune" ? (
            <p className="note">{t("docker.noOptions")}</p>
          ) : null}
        </>
      }
      output={
        <OutputPane
          running={runner.running}
          error={runner.error}
          meta={
            out
              ? {
                  success: out.result.success,
                  exitCode: out.result.exit_code,
                  stderr: out.result.stderr,
                }
              : null
          }
          ranAt={runner.ranAt}
          durationMs={runner.durationMs}
          ranCommand={runner.ranCommand}
          stale={runner.ranCommand !== null && runner.ranCommand !== form.preview}
          emptyHint={t("docker.empty")}
        >
          {out?.kind === "du" ? <DockerDuResultView result={out.result} /> : null}
          {out?.kind === "containers" ? <DockerContainersResultView result={out.result} /> : null}
          {out?.kind === "images" ? <DockerImagesResultView result={out.result} /> : null}
          {out?.kind === "networks" ? <DockerNetworksResultView result={out.result} /> : null}
          {out?.kind === "volumes" ? <DockerVolumesResultView result={out.result} /> : null}
          {out?.kind === "sysdf" ? <DockerSystemDfResultView result={out.result} /> : null}
          {out?.kind === "sysinfo" ? <DockerSystemInfoResultView result={out.result} /> : null}
          {out?.kind === "ls" ? <DockerLsResultView result={out.result} /> : null}
          {out?.kind === "version" ? <DockerVersionResultView result={out.result} /> : null}
          {out?.kind === "inspect" ? <DockerInspectResultView result={out.result} /> : null}
          {out?.kind === "prune" ? <CommandResultView result={out.result} /> : null}
        </OutputPane>
      }
    />
  );
}
