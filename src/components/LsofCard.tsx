import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { LSOF_PROTOCOLS, LSOF_STATES } from "../commands/lsofOptions";
import { useLsof } from "../hooks/useLsof";
import { LsofResultView } from "./LsofResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import { OptionRow, Segmented, TextField } from "./ui/controls";

export function LsofCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useLsof();
  const { runner } = form;
  const udpLocked = form.protocol === "UDP";

  const blocker = form.pidList === null
    ? t("lsof.pidsInvalid")
    : !form.userOk
      ? t("lsof.userInvalid")
      : !form.commOk
        ? t("lsof.commInvalid")
        : !form.portOk
          ? t("lsof.portInvalid")
          : !form.hostOk
            ? t("lsof.hostInvalid")
            : form.stateConflict
              ? t("lsof.stateConflict")
              : !form.hasSelector
                ? t("lsof.selectorRequired")
                : null;

  return (
    <Workbench
      active={active}
      title={t("lsof.title")}
      description={t("lsof.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("lsof.execute")}
      runningLabel={t("lsof.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("lsof.groupProcess")}</div>
          <OptionRow
            id="pids"
            label={t("lsof.pids")}
            flag="-p"
            on={form.pids.trim() !== ""}
            hint={t("lsof.hintPids")}
            error={form.pidList === null ? t("lsof.pidsInvalid") : null}
          >
            <TextField
              value={form.pids}
              onChange={form.setPids}
              placeholder={t("lsof.pidsPlaceholder")}
              invalid={form.pidList === null}
              onClear={() => form.setPids("")}
            />
          </OptionRow>
          <OptionRow
            id="user"
            label={t("lsof.user")}
            flag="-u"
            on={form.user.trim() !== ""}
            hint={t("lsof.hintUser")}
            error={form.userOk ? null : t("lsof.userInvalid")}
          >
            <TextField
              value={form.user}
              onChange={form.setUser}
              placeholder={t("lsof.userPlaceholder")}
              invalid={!form.userOk}
              onClear={() => form.setUser("")}
            />
          </OptionRow>
          <OptionRow
            id="comm"
            label={t("lsof.comm")}
            flag="-c"
            on={form.comm.trim() !== ""}
            hint={t("lsof.hintComm")}
            error={form.commOk ? null : t("lsof.commInvalid")}
          >
            <TextField
              value={form.comm}
              onChange={form.setComm}
              placeholder={t("lsof.commPlaceholder")}
              invalid={!form.commOk}
              onClear={() => form.setComm("")}
            />
          </OptionRow>

          <div className="group-label">{t("lsof.groupNetwork")}</div>
          <OptionRow
            id="protocol"
            label={t("lsof.protocol")}
            flag="-i"
            on={form.protocol !== "any"}
            hint={t("lsof.hintProtocol")}
          >
            <Segmented
              value={form.protocol}
              onChange={form.pickProtocol}
              options={LSOF_PROTOCOLS.map((protocol) => ({
                value: protocol,
                label: t(`lsofProtocol.${protocol}`),
              }))}
            />
          </OptionRow>
          <OptionRow
            id="port"
            label={t("lsof.port")}
            flag="-i :port"
            on={form.port.trim() !== ""}
            hint={t("lsof.hintPort")}
            error={form.portOk ? null : t("lsof.portInvalid")}
          >
            <TextField
              value={form.port}
              onChange={form.setPort}
              placeholder={t("lsof.portPlaceholder")}
              invalid={!form.portOk}
              onClear={() => form.setPort("")}
            />
          </OptionRow>
          <OptionRow
            id="host"
            label={t("lsof.host")}
            flag="-i @host"
            on={form.host.trim() !== ""}
            hint={t("lsof.hintHost")}
            error={form.hostOk ? null : t("lsof.hostInvalid")}
          >
            <TextField
              value={form.host}
              onChange={form.setHost}
              placeholder={t("lsof.hostPlaceholder")}
              invalid={!form.hostOk}
              onClear={() => form.setHost("")}
            />
          </OptionRow>
          <OptionRow
            id="state"
            label={t("lsof.state")}
            flag="-s"
            on={form.state !== "any"}
            hint={udpLocked ? t("lsof.stateLocked") : t("lsof.hintState")}
          >
            <Segmented
              value={form.state}
              onChange={form.setState}
              options={LSOF_STATES.map((state) => ({
                value: state,
                label: t(`lsofState.${state}`),
              }))}
            />
          </OptionRow>
          <p className="note">{t("lsof.note")}</p>
        </>
      }
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
          stale={runner.ranCommand !== null && runner.ranCommand !== form.preview}
          emptyHint={t("lsof.empty")}
        >
          {runner.result ? <LsofResultView result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
