import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { KILL_MAX_PIDS, KILL_SIGNALS } from "../commands/killOptions";
import { useKill } from "../hooks/useKill";
import { KillResultView } from "./KillResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, TextField } from "./ui/controls";

export function KillCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useKill();
  const { runner } = form;
  const found = form.targets.filter((target) => target.found);

  const blocker = !form.pidsOk
    ? t("kill.pidsInvalid", { max: KILL_MAX_PIDS })
    : form.targetsError
      ? form.targetsError
      : !form.anyFound
        ? t("kill.noTargets")
        : null;

  // 実行ボタン (⌘↵ 含む) は確認パネルを開くだけ。送信はパネルのボタンでのみ行う。
  const notice = form.confirming ? (
    <div className="kill-confirm">
      <span>
        ⚠️{" "}
        {t("kill.confirm", {
          signal: form.signal,
          n: found.length,
          names: found.map((target) => `${target.pid} ${shortName(target.command)}`).join(", "),
        })}
      </span>
      <div className="kill-confirm-actions">
        <button type="button" className="btn-ghost" onClick={form.cancelConfirm}>
          {t("kill.cancel")}
        </button>
        <button type="button" className="btn-run is-danger" onClick={form.execute}>
          {t("kill.send", { signal: form.signal })}
        </button>
      </div>
    </div>
  ) : null;

  return (
    <Workbench
      active={active}
      title={t("kill.title")}
      description={t("kill.description")}
      tokens={form.tokens}
      onRun={form.requestConfirm}
      canRun={form.valid && !form.confirming}
      running={runner.running}
      runLabel={t("kill.execute")}
      runningLabel={t("kill.executing")}
      danger
      blocker={blocker}
      notice={notice}
      about={about}
      options={
        <>
          <div className="group-label">{t("kill.groupTarget")}</div>
          <OptionRow
            id="pids"
            label={t("kill.pids")}
            on={form.pids.trim() !== ""}
            hint={t("kill.hintPids", { max: KILL_MAX_PIDS })}
            error={form.pids.trim() === "" || form.pidsOk ? null : t("kill.pidsInvalid", { max: KILL_MAX_PIDS })}
          >
            <TextField
              value={form.pids}
              onChange={form.setPids}
              placeholder={t("kill.pidsPlaceholder")}
              invalid={form.pids.trim() !== "" && !form.pidsOk}
              onClear={() => form.setPids("")}
            />
          </OptionRow>
          {form.targets.length > 0 ? (
            <OptionRow id="targets" label={t("kill.targets")} on={form.anyFound} hint={t("kill.hintTargets")}>
              <div className="table-wrap">
                <table className="top-table">
                  <thead>
                    <tr>
                      <th>PID</th>
                      <th>{t("kill.user")}</th>
                      <th>{t("kill.command")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {form.targets.map((target) => (
                      <tr key={target.pid}>
                        <td className="mono">{target.pid}</td>
                        <td className="mono">{target.found ? target.user : "—"}</td>
                        <td className="mono" title={target.command}>
                          {target.found ? shortName(target.command) : t("kill.notFound")}
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              <div>
                <button type="button" className="link" onClick={form.refreshTargets}>
                  {t("kill.refresh")}
                </button>
              </div>
            </OptionRow>
          ) : null}

          <div className="group-label">{t("kill.groupSignal")}</div>
          <OptionRow id="signal" label={t("kill.signal")} flag="-s" on hint={t(`killSignalHelp.${form.signal}`)}>
            <Segmented
              value={form.signal}
              onChange={form.setSignal}
              wrap
              options={KILL_SIGNALS.map((signal) => ({
                value: signal,
                label: signal,
                sub: t(`killSignal.${signal}`),
              }))}
            />
          </OptionRow>
          <FlagRow
            id="sudo"
            label={t("killFlags.sudo")}
            flag="sudo"
            hint={t("kill.hintSudo")}
            checked={form.sudo}
            onChange={form.setSudo}
          />
          <p className="note">{t("kill.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("kill.empty")}
          result={runner.result}
        >
          {runner.result ? <KillResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}

// ps の comm はフルパスのことがあるので末尾だけ見せる。
function shortName(command: string): string {
  const parts = command.split("/");
  return parts[parts.length - 1] || command;
}
