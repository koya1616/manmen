import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { NETWORKSETUP_SUBS, useNetworksetup } from "../hooks/useNetworksetup";
import { NetworksetupResultView } from "./NetworksetupResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { OptionRow, Segmented, TextField } from "./ui/controls";

export function NetworksetupCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useNetworksetup();
  const { runner } = form;

  const blocker = !form.serviceOk
    ? (form.servicesError ?? t("networksetup.serviceInvalid"))
    : !form.deviceOk
      ? t("networksetup.deviceInvalid")
      : null;

  return (
    <Workbench
      active={active}
      title={t("networksetup.title")}
      description={t("networksetup.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("networksetup.execute")}
      runningLabel={t("networksetup.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <OptionRow id="sub" label={t("networksetup.sub")} on>
            <Segmented
              value={form.sub}
              onChange={form.setSub}
              wrap
              options={NETWORKSETUP_SUBS.map((sub) => ({
                value: sub,
                label: t(`networksetupSub.${sub}`),
              }))}
            />
            <p className="opt-hint">{t(`networksetupSubHelp.${form.sub}`)}</p>
          </OptionRow>

          {form.sub === "info" ? (
            <OptionRow
              id="service"
              label={t("networksetup.service")}
              on
              hint={t("networksetup.hintService")}
              error={form.serviceOk ? null : t("networksetup.serviceInvalid")}
            >
              {form.services.length === 0 ? (
                <p className="muted">{form.servicesError ?? t("networksetup.loadingServices")}</p>
              ) : (
                <Segmented
                  value={form.service}
                  onChange={form.setService}
                  wrap
                  options={form.services.map((name) => ({ value: name, label: name }))}
                />
              )}
            </OptionRow>
          ) : null}

          {form.sub === "wifi" ? (
            <OptionRow
              id="device"
              label={t("networksetup.device")}
              on={form.device.trim() !== ""}
              hint={t("networksetup.hintDevice")}
              error={form.deviceOk ? null : t("networksetup.deviceInvalid")}
            >
              <TextField
                value={form.device}
                onChange={form.setDevice}
                placeholder={t("networksetup.devicePlaceholder")}
                invalid={!form.deviceOk}
                onClear={() => form.setDevice("")}
              />
            </OptionRow>
          ) : null}
          <p className="note">{t("networksetup.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("networksetup.empty")}
          result={runner.result}
        >
          {runner.result ? <NetworksetupResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
