import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useIfconfig } from "../hooks/useIfconfig";
import { IfconfigResultView } from "./IfconfigResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { FlagRow, OptionRow, Segmented, TextField } from "./ui/controls";

export function IfconfigCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useIfconfig();
  const { runner } = form;

  return (
    <Workbench
      active={active}
      title={t("ifconfig.title")}
      description={t("ifconfig.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("ifconfig.execute")}
      runningLabel={t("ifconfig.executing")}
      blocker={form.ifaceOk ? null : t("ifconfig.interfaceInvalid")}
      about={about}
      options={
        <>
          <div className="group-label">{t("ifconfig.groupTarget")}</div>
          <OptionRow
            id="interface"
            label={t("ifconfig.interface")}
            flag={form.iface.trim() === "" ? "-a" : undefined}
            on
            hint={t("ifconfig.hintInterface")}
            error={form.ifaceOk ? null : t("ifconfig.interfaceInvalid")}
          >
            <TextField
              value={form.iface}
              onChange={form.setIface}
              placeholder={t("ifconfig.interfacePlaceholder")}
              invalid={!form.ifaceOk}
              onClear={() => form.setIface("")}
              suggestions={["en0", "en1", "lo0", "utun0", "bridge0"]}
            />
          </OptionRow>
          {form.iface.trim() === "" ? (
            <FlagRow
              id="upOnly"
              label={t("ifconfigFlags.upOnly")}
              flag="-u"
              hint={t("ifconfig.hintUpOnly")}
              checked={form.upOnly}
              onChange={form.setUpOnly}
            />
          ) : null}
          <OptionRow
            id="family"
            label={t("ifconfig.family")}
            on={form.family !== ""}
            hint={t("ifconfig.hintFamily")}
          >
            <Segmented
              value={form.family}
              onChange={form.setFamily}
              options={[
                { value: "", label: t("ifconfig.allFamilies") },
                { value: "inet", label: "IPv4", sub: "inet" },
                { value: "inet6", label: "IPv6", sub: "inet6" },
              ]}
            />
          </OptionRow>

          <div className="group-label">{t("ifconfig.groupDisplay")}</div>
          <FlagRow
            id="activeOnly"
            label={t("ifconfigFlags.activeOnly")}
            flag={t("df.displayOnly")}
            hint={t("ifconfig.hintActiveOnly")}
            checked={form.activeOnly}
            onChange={form.setActiveOnly}
          />
          <p className="note">{t("ifconfig.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("ifconfig.empty")}
          result={runner.result}
        >
          {runner.result ? (
            <IfconfigResultView result={runner.result} activeOnly={form.activeOnly} />
          ) : null}
        </RunnerOutput>
      }
    />
  );
}
