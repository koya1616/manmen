import { useState, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  SSH_MAX_TIMEOUT,
  SSH_MIN_TIMEOUT,
  SSH_STRICTS,
  type SshStrict,
  type SshTransport,
} from "../commands/sshOptions";
import { TUNNEL_MODES, type TunnelMode } from "../commands/sshTunnelOptions";
import { useSsh } from "../hooks/useSsh";
import { useSshHosts } from "../hooks/useSshHosts";
import { useSshTunnel } from "../hooks/useSshTunnel";
import type { SshKnownHost } from "../types";
import { SshResultView } from "./SshResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, Stepper, TextField } from "./ui/controls";

const TRANSPORTS: SshTransport[] = ["auto", "4", "6"];

type SshPurpose = "config" | "test" | "tunnel";

// 入力が `~/.ssh/config` の別名と一致したら解決先を表示する
function KnownHostHint({ hosts, value }: { hosts: SshKnownHost[]; value: string }) {
  const { t } = useTranslation();
  const found = hosts.find((host) => host.alias === value.trim());
  if (!found || value.trim() === "") return null;
  const detail = [
    found.user ?? undefined,
    found.hostname ?? found.alias,
    found.port !== null ? String(found.port) : undefined,
  ]
    .filter(Boolean)
    .join(" ");
  return <p className="opt-hint">{t("ssh.knownHostDetail", { detail })}</p>;
}

export function SshCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const [purpose, setPurpose] = useState<SshPurpose>("config");
  const ssh = useSsh();
  const tun = useSshTunnel(active);
  const { hosts } = useSshHosts(active);
  const aliases = hosts.map((host) => host.alias);
  const isTunnel = purpose === "tunnel";
  const isTest = ssh.mode === "test";

  function pickPurpose(next: SshPurpose) {
    setPurpose(next);
    if (next === "config" || next === "test") {
      ssh.setMode(next);
    }
  }

  const sshBlocker = !ssh.hostOk
    ? t("ssh.hostInvalid")
    : !ssh.userOk
      ? t("ssh.userInvalid")
      : !ssh.portOk
        ? t("ssh.portInvalid")
        : !ssh.timeoutOk
          ? t("ssh.timeoutInvalid")
          : null;

  const tunnelBlocker = !tun.hostOk
    ? t("sshTunnel.hostInvalid")
    : !tun.userOk
      ? t("sshTunnel.userInvalid")
      : !tun.portOk
        ? t("sshTunnel.portInvalid")
        : !tun.bindOk
          ? t("sshTunnel.bindInvalid")
          : !tun.localPortOk
            ? t("sshTunnel.localPortInvalid")
            : !tun.remoteHostOk
              ? t("sshTunnel.remoteHostInvalid")
              : !tun.remotePortOk
                ? t("sshTunnel.remotePortInvalid")
                : null;

  const tokens = isTunnel ? tun.tokens : ssh.tokens;
  const preview = isTunnel ? tun.preview : ssh.preview;

  return (
    <Workbench
      active={active}
      title={t(isTunnel ? "sshTunnel.title" : "ssh.title")}
      description={t(isTunnel ? "sshTunnel.description" : "ssh.description")}
      tokens={tokens}
      onRun={isTunnel ? tun.start : ssh.execute}
      canRun={isTunnel ? tun.valid && !tun.starting : ssh.valid}
      running={isTunnel ? tun.starting : ssh.runner.running}
      runLabel={t(isTunnel ? "sshTunnel.start" : "ssh.execute")}
      runningLabel={t(isTunnel ? "sshTunnel.starting" : "ssh.executing")}
      blocker={isTunnel ? tunnelBlocker : sshBlocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("ssh.groupPurpose")}</div>
          <OptionRow id="purpose" label={t("ssh.purpose")} on hint={t("ssh.hintPurpose")}>
            <Segmented
              value={purpose}
              onChange={pickPurpose}
              options={(["config", "test", "tunnel"] as SshPurpose[]).map((mode) => ({
                value: mode,
                label: t(`sshPurpose.${mode}`),
                sub: mode === "config" ? "-G" : mode === "test" ? "exit" : "-N",
              }))}
            />
          </OptionRow>

          {!isTunnel ? (
            <>
              <div className="group-label">{t("ssh.groupDestination")}</div>
              <OptionRow
                id="destination"
                label={t("ssh.destination")}
                on={ssh.host.trim() !== "" || ssh.user.trim() !== ""}
                hint={t("ssh.hintDestination")}
                error={ssh.hostOk && ssh.userOk ? null : t("ssh.hostInvalid")}
              >
                <TextField
                  value={ssh.host}
                  onChange={ssh.setHost}
                  placeholder={t("ssh.hostPlaceholder")}
                  invalid={!ssh.hostOk}
                  onClear={() => ssh.setHost("")}
                  suggestions={aliases}
                />
                <KnownHostHint hosts={hosts} value={ssh.host} />
                <TextField
                  value={ssh.user}
                  onChange={ssh.setUser}
                  placeholder={t("ssh.userPlaceholder")}
                  invalid={!ssh.userOk}
                  onClear={() => ssh.setUser("")}
                />
              </OptionRow>
              <OptionRow
                id="port"
                label={t("ssh.port")}
                flag="-p"
                on={ssh.port.trim() !== ""}
                hint={t("ssh.hintPort")}
                error={ssh.portOk ? null : t("ssh.portInvalid")}
              >
                <TextField
                  value={ssh.port}
                  onChange={ssh.setPort}
                  placeholder={t("ssh.portPlaceholder")}
                  invalid={!ssh.portOk}
                  onClear={() => ssh.setPort("")}
                />
              </OptionRow>

              {isTest && (
                <>
                  <OptionRow
                    id="timeout"
                    label={t("ssh.timeout")}
                    flag="ConnectTimeout"
                    on
                    hint={t("ssh.hintTimeout")}
                    error={ssh.timeoutOk ? null : t("ssh.timeoutInvalid")}
                  >
                    <Stepper
                      value={ssh.timeout}
                      onChange={ssh.setTimeout}
                      min={SSH_MIN_TIMEOUT}
                      max={SSH_MAX_TIMEOUT}
                      step={5}
                      unit={t("ssh.secondsUnit")}
                      presets={["5", "10", "30"].map((v) => ({ value: v, label: v }))}
                    />
                  </OptionRow>
                  <OptionRow
                    id="strict"
                    label={t("ssh.strict")}
                    flag="StrictHostKeyChecking"
                    on
                    hint={t("ssh.hintStrict")}
                  >
                    <Segmented
                      value={ssh.strict}
                      onChange={(next) => ssh.setStrict(next as SshStrict)}
                      options={SSH_STRICTS.map((strict) => ({
                        value: strict,
                        label: t(`sshStrict.${strict}`),
                      }))}
                    />
                  </OptionRow>
                </>
              )}

              <div className="group-label">{t("ssh.groupDisplay")}</div>
              <FlagRow
                id="verbose"
                label={t("sshFlags.verbose")}
                flag="-v"
                hint={t("ssh.hintVerbose")}
                checked={ssh.verbose}
                onChange={ssh.setVerbose}
              />
              <OptionRow
                id="transport"
                label={t("ssh.transport")}
                flag="-4 / -6"
                on={ssh.transport !== "auto"}
                hint={t("ssh.hintTransport")}
              >
                <Segmented
                  value={ssh.transport}
                  onChange={ssh.setTransport}
                  options={TRANSPORTS.map((mode) => ({
                    value: mode,
                    label: t(`sshTransport.${mode}`),
                  }))}
                />
              </OptionRow>
              <p className="note">{t("ssh.note")}</p>
            </>
          ) : (
            <>
              <div className="group-label">{t("sshTunnel.groupDirection")}</div>
              <OptionRow
                id="tunnelMode"
                label={t("sshTunnel.direction")}
                on
                hint={t("sshTunnel.hintDirection")}
              >
                <Segmented
                  value={tun.tunnelMode}
                  onChange={(next) => tun.setTunnelMode(next as TunnelMode)}
                  options={TUNNEL_MODES.map((mode) => ({
                    value: mode,
                    label: t(`sshTunnelMode.${mode}`),
                    sub: mode === "local" ? "-L" : mode === "remote" ? "-R" : "-D",
                  }))}
                />
              </OptionRow>

              <div className="group-label">{t("sshTunnel.groupForward")}</div>
              <OptionRow
                id="bind"
                label={t("sshTunnel.bind")}
                on={tun.localHost.trim() !== ""}
                hint={t("sshTunnel.hintBind")}
                error={tun.bindOk ? null : t("sshTunnel.bindInvalid")}
              >
                <TextField
                  value={tun.localHost}
                  onChange={tun.setLocalHost}
                  placeholder={t("sshTunnel.bindPlaceholder")}
                  invalid={!tun.bindOk}
                  onClear={() => tun.setLocalHost("")}
                />
              </OptionRow>
              <OptionRow
                id="localPort"
                label={t("sshTunnel.localPort")}
                on
                hint={t("sshTunnel.hintLocalPort")}
                error={tun.localPortOk ? null : t("sshTunnel.localPortInvalid")}
              >
                <TextField
                  value={tun.localPort}
                  onChange={tun.setLocalPort}
                  placeholder={t("sshTunnel.localPortPlaceholder")}
                  invalid={!tun.localPortOk}
                  onClear={() => tun.setLocalPort("")}
                />
              </OptionRow>
              {tun.tunnelMode !== "dynamic" && (
                <>
                  <OptionRow
                    id="remoteHost"
                    label={t("sshTunnel.remoteHost")}
                    on={tun.remoteHost.trim() !== ""}
                    hint={t("sshTunnel.hintRemoteHost")}
                    error={tun.remoteHostOk ? null : t("sshTunnel.remoteHostInvalid")}
                  >
                    <TextField
                      value={tun.remoteHost}
                      onChange={tun.setRemoteHost}
                      placeholder={t("sshTunnel.remoteHostPlaceholder")}
                      invalid={!tun.remoteHostOk}
                      onClear={() => tun.setRemoteHost("")}
                    />
                  </OptionRow>
                  <OptionRow
                    id="remotePort"
                    label={t("sshTunnel.remotePort")}
                    on={tun.remotePort.trim() !== ""}
                    hint={t("sshTunnel.hintRemotePort")}
                    error={tun.remotePortOk ? null : t("sshTunnel.remotePortInvalid")}
                  >
                    <TextField
                      value={tun.remotePort}
                      onChange={tun.setRemotePort}
                      placeholder={t("sshTunnel.remotePortPlaceholder")}
                      invalid={!tun.remotePortOk}
                      onClear={() => tun.setRemotePort("")}
                    />
                  </OptionRow>
                </>
              )}

              <div className="group-label">{t("sshTunnel.groupDestination")}</div>
              <OptionRow
                id="tunnelDestination"
                label={t("sshTunnel.destination")}
                on={tun.host.trim() !== "" || tun.user.trim() !== ""}
                hint={t("sshTunnel.hintDestination")}
                error={tun.hostOk && tun.userOk ? null : t("sshTunnel.hostInvalid")}
              >
                <TextField
                  value={tun.host}
                  onChange={tun.setHost}
                  placeholder={t("sshTunnel.hostPlaceholder")}
                  invalid={!tun.hostOk}
                  onClear={() => tun.setHost("")}
                  suggestions={aliases}
                />
                <KnownHostHint hosts={hosts} value={tun.host} />
                <TextField
                  value={tun.user}
                  onChange={tun.setUser}
                  placeholder={t("sshTunnel.userPlaceholder")}
                  invalid={!tun.userOk}
                  onClear={() => tun.setUser("")}
                />
              </OptionRow>
              <OptionRow
                id="tunnelPort"
                label={t("sshTunnel.port")}
                flag="-p"
                on={tun.port.trim() !== ""}
                hint={t("sshTunnel.hintPort")}
                error={tun.portOk ? null : t("sshTunnel.portInvalid")}
              >
                <TextField
                  value={tun.port}
                  onChange={tun.setPort}
                  placeholder={t("sshTunnel.portPlaceholder")}
                  invalid={!tun.portOk}
                  onClear={() => tun.setPort("")}
                />
              </OptionRow>

              <div className="group-label">{t("sshTunnel.groupDisplay")}</div>
              <FlagRow
                id="keepalive"
                label={t("sshTunnelFlags.keepalive")}
                flag="ServerAlive"
                hint={t("sshTunnel.hintKeepalive")}
                checked={tun.keepalive}
                onChange={tun.setKeepalive}
              />
              <OptionRow
                id="tunnelTransport"
                label={t("sshTunnel.transport")}
                flag="-4 / -6"
                on={tun.transport !== "auto"}
                hint={t("sshTunnel.hintTransport")}
              >
                <Segmented
                  value={tun.transport}
                  onChange={tun.setTransport}
                  options={TRANSPORTS.map((mode) => ({
                    value: mode,
                    label: t(`sshTransport.${mode}`),
                  }))}
                />
              </OptionRow>
              <p className="note">{t("sshTunnel.note")}</p>
              <p className="note">{t("sshTunnel.noteClose")}</p>
            </>
          )}
        </>
      }
      output={
        isTunnel ? (
          <OutputPane
            running={tun.starting}
            error={tun.error}
            meta={null}
            ranAt={null}
            durationMs={null}
            ranCommand={null}
            stale={false}
            emptyHint={t("sshTunnel.empty")}
          >
            {tun.tunnels.length === 0 ? null : (
              <div className="table-wrap">
                <table className="top-table">
                  <thead>
                    <tr>
                      <th>{t("sshTunnelResult.summary")}</th>
                      <th>{t("sshTunnelResult.command")}</th>
                      <th>{t("sshTunnelResult.action")}</th>
                    </tr>
                  </thead>
                  <tbody>
                    {tun.tunnels.map((tunnel) => (
                      <tr key={tunnel.id}>
                        <td className="mono">{tunnel.summary}</td>
                        <td className="mono">{tunnel.command}</td>
                        <td>
                          <button
                            type="button"
                            className="btn-ghost"
                            disabled={tun.stoppingId === tunnel.id}
                            onClick={() => tun.stop(tunnel.id)}
                          >
                            {t("sshTunnel.stop")}
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            )}
          </OutputPane>
        ) : (
          <OutputPane
            running={ssh.runner.running}
            error={ssh.runner.error}
            meta={
              ssh.runner.result
                ? {
                    success: ssh.runner.result.success,
                    exitCode: ssh.runner.result.exit_code,
                    stderr: ssh.runner.result.stderr,
                  }
                : null
            }
            ranAt={ssh.runner.ranAt}
            durationMs={ssh.runner.durationMs}
            ranCommand={ssh.runner.ranCommand}
            stale={ssh.runner.ranCommand !== null && ssh.runner.ranCommand !== preview}
            emptyHint={t("ssh.empty")}
          >
            {ssh.runner.result ? <SshResultView result={ssh.runner.result} /> : null}
          </OutputPane>
        )
      }
    />
  );
}
