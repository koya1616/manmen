import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { PS_COLUMNS, type PsSort } from "../commands/psOptions";
import { usePs } from "../hooks/usePs";
import { PsResultView } from "./PsResultView";
import { RunnerOutput } from "./ui/RunnerOutput";
import { Workbench } from "./ui/Workbench";
import { KeyPicker, OptionRow, Segmented, TextField } from "./ui/controls";

const SORTS: PsSort[] = ["none", "cpu", "mem"];
const FEATURED_COLUMNS = ["pid", "user", "%cpu", "%mem", "rss", "time", "state", "command"];

export function PsCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = usePs();
  const { runner } = form;
  const labelOf = (key: string) => t(`psColumns.${key}`);
  const helpOf = (key: string) => t(`psHelp.${key}`);

  const blocker = !form.userOk
    ? t("ps.userInvalid")
    : form.pidList === null
      ? t("ps.pidsInvalid")
      : !form.columnsOk
        ? t("ps.columnsInvalid")
        : null;

  return (
    <Workbench
      active={active}
      title={t("ps.title")}
      description={t("ps.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("ps.execute")}
      runningLabel={t("ps.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("ps.groupDisplay")}</div>
          <OptionRow
            id="columns"
            label={t("ps.columns")}
            flag="-o"
            on={form.columns.length > 0}
            hint={
              <>
                {t("ps.columnsHint")}
                <button type="button" className="link" onClick={form.resetColumns}>
                  {t("ui.reset")}
                </button>
              </>
            }
            error={form.columnsOk ? null : t("ps.columnsInvalid")}
          >
            <KeyPicker
              keys={PS_COLUMNS}
              featured={FEATURED_COLUMNS}
              selected={form.columns}
              onPick={form.toggleColumn}
              labelOf={labelOf}
              helpOf={helpOf}
              multi
            />
          </OptionRow>
          <OptionRow
            id="sort"
            label={t("ps.sort")}
            flag="-r / -m"
            on={form.sort !== "none"}
            hint={t("ps.hintSort")}
          >
            <Segmented
              value={form.sort}
              onChange={form.setSort}
              options={SORTS.map((sort) => ({
                value: sort,
                label: t(`psSort.${sort}`),
              }))}
            />
          </OptionRow>

          <div className="group-label">{t("ps.groupFilter")}</div>
          <OptionRow
            id="user"
            label={t("ps.user")}
            flag="-U"
            on={form.user.trim() !== ""}
            hint={t("ps.hintUser")}
            error={form.userOk ? null : t("ps.userInvalid")}
          >
            <TextField
              value={form.user}
              onChange={form.setUser}
              placeholder={t("ps.userPlaceholder")}
              invalid={!form.userOk}
              onClear={() => form.setUser("")}
            />
          </OptionRow>
          <OptionRow
            id="pids"
            label={t("ps.pids")}
            flag="-p"
            on={form.pids.trim() !== ""}
            hint={t("ps.hintPids")}
            error={form.pidList === null ? t("ps.pidsInvalid") : null}
          >
            <TextField
              value={form.pids}
              onChange={form.setPids}
              placeholder={t("ps.pidsPlaceholder")}
              invalid={form.pidList === null}
              onClear={() => form.setPids("")}
            />
          </OptionRow>
          <p className="note">{t("ps.note")}</p>
        </>
      }
      output={
        <RunnerOutput
          runner={runner}
          preview={form.preview}
          emptyHint={t("ps.empty")}
          result={runner.result}
        >
          {runner.result ? <PsResultView result={runner.result} /> : null}
        </RunnerOutput>
      }
    />
  );
}
