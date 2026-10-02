import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import {
  TOP_KEYS,
  TOP_MAX_COUNT,
  TOP_MAX_NCOLS,
  TOP_MAX_STATS,
  TOP_MIN_NCOLS,
  type TopCountMode,
  type TopKey,
  type TopSortOrder,
} from "../commands/topOptions";
import { useTop } from "../hooks/useTop";
import { TopResultView } from "./TopResultView";
import { OutputPane } from "./ui/OutputPane";
import { Workbench } from "./ui/Workbench";
import {
  FlagRow,
  KeyPicker,
  OptionRow,
  Segmented,
  Stepper,
  TextField,
} from "./ui/controls";

const ORDERS: TopSortOrder[] = ["", "+", "-"];
const MODES: TopCountMode[] = ["n", "a", "d", "e"];
const FEATURED_SORT: TopKey[] = ["cpu", "mem", "time", "pid", "command", "threads"];
const FEATURED_STATS: TopKey[] = ["pid", "command", "cpu", "mem", "time", "threads", "user", "state"];

export function TopCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const form = useTop();
  const { runner } = form;
  const labelOf = (key: string) => t(`topSort.${key}`);
  const helpOf = (key: string) => t(`topHelp.${key}`);

  const blocker = !form.isValidCount
    ? t("top.countInvalid")
    : !form.userOk
      ? t("top.userInvalid")
      : form.pidList === null
        ? t("top.pidsInvalid")
        : !form.ncolsOk
          ? t("top.ncolsInvalid")
          : null;

  return (
    <Workbench
      active={active}
      title={t("top.title")}
      description={t("top.description")}
      tokens={form.tokens}
      onRun={form.execute}
      canRun={form.valid}
      running={runner.running}
      runLabel={t("top.execute")}
      runningLabel={t("top.executing")}
      blocker={blocker}
      about={about}
      options={
        <>
          <div className="group-label">{t("top.groupSort")}</div>
          <OptionRow id="sort" label={t("top.sort")} flag="-o" on>
            <KeyPicker
              keys={TOP_KEYS}
              featured={FEATURED_SORT}
              selected={[form.sortKey]}
              onPick={(key) => form.setSortKey(key as TopKey)}
              labelOf={labelOf}
              helpOf={helpOf}
            />
            <Segmented
              value={form.sortOrder}
              onChange={form.setSortOrder}
              options={ORDERS.map((order) => ({
                value: order,
                label: t(`topOrder.${order || "default"}`),
              }))}
            />
          </OptionRow>
          <OptionRow
            id="secondary"
            label={t("top.secondary")}
            flag="-O"
            on={form.secondaryKey !== ""}
            hint={t("top.hintSecondary")}
          >
            <KeyPicker
              keys={TOP_KEYS}
              featured={FEATURED_SORT}
              selected={form.secondaryKey ? [form.secondaryKey] : []}
              onPick={(key) => form.setSecondaryKey(form.secondaryKey === key ? "" : key)}
              labelOf={labelOf}
              helpOf={helpOf}
            />
          </OptionRow>

          <div className="group-label">{t("top.groupFilter")}</div>
          <OptionRow id="count" label={t("top.count")} flag="-n" on hint={t("top.hintCount")}>
            <Stepper
              value={form.count}
              onChange={form.setCount}
              min={1}
              max={TOP_MAX_COUNT}
              step={5}
              presets={["10", "20", "50", "100"].map((v) => ({ value: v, label: v }))}
            />
          </OptionRow>
          <OptionRow
            id="user"
            label={t("top.user")}
            flag="-user"
            on={form.user.trim() !== ""}
            hint={t("top.hintUser")}
            error={form.userOk ? null : t("top.userInvalid")}
          >
            <TextField
              value={form.user}
              onChange={form.setUser}
              placeholder={t("top.userPlaceholder")}
              invalid={!form.userOk}
              onClear={() => form.setUser("")}
            />
          </OptionRow>
          <OptionRow
            id="pids"
            label={t("top.pids")}
            flag="-pid"
            on={form.pids.trim() !== ""}
            hint={t("top.hintPids")}
            error={form.pidList === null ? t("top.pidsInvalid") : null}
          >
            <TextField
              value={form.pids}
              onChange={form.setPids}
              placeholder={t("top.pidsPlaceholder")}
              invalid={form.pidList === null}
              onClear={() => form.setPids("")}
            />
          </OptionRow>

          <div className="group-label">{t("top.groupDisplay")}</div>
          <OptionRow
            id="stats"
            label={t("top.stats")}
            flag="-stats"
            on={form.stats.length > 0}
            hint={
              <>
                {t("top.statsHint", { max: TOP_MAX_STATS })}
                {form.stats.length > 0 ? (
                  <button type="button" className="link" onClick={form.clearStats}>
                    {t("ui.reset")}
                  </button>
                ) : null}
              </>
            }
          >
            <KeyPicker
              keys={TOP_KEYS}
              featured={FEATURED_STATS}
              selected={form.stats}
              onPick={form.toggleStat}
              labelOf={labelOf}
              helpOf={helpOf}
              multi
            />
          </OptionRow>
          <OptionRow
            id="mode"
            label={t("top.mode")}
            flag="-c"
            on={form.countMode !== "n"}
            hint={t(`topModeHelp.${form.countMode}`)}
          >
            <Segmented
              value={form.countMode}
              onChange={form.setCountMode}
              wrap
              options={MODES.map((mode) => ({
                value: mode,
                label: t(`topMode.${mode}`),
                sub: mode === "n" ? undefined : `-c ${mode}`,
              }))}
            />
          </OptionRow>
          <FlagRow
            id="frameworks"
            label={t("topFlags.frameworks")}
            flag="-F"
            hint={t("top.hintFrameworks")}
            checked={form.noFrameworks}
            onChange={form.setNoFrameworks}
          />
          <FlagRow
            id="memoryMap"
            label={t("topFlags.memoryMap")}
            flag="-r"
            hint={t("top.hintMemoryMap")}
            checked={form.memoryMap}
            onChange={form.setMemoryMap}
          />
          <FlagRow
            id="swap"
            label={t("topFlags.swap")}
            flag="-S"
            hint={t("top.hintSwap")}
            checked={form.swap}
            onChange={form.setSwap}
          />
          <OptionRow
            id="ncols"
            label={t("top.ncols")}
            flag="-ncols"
            on={form.ncols.trim() !== ""}
            hint={t("top.hintNcols")}
            error={form.ncolsOk ? null : t("top.ncolsInvalid")}
          >
            <Stepper
              value={form.ncols}
              onChange={form.setNcols}
              min={TOP_MIN_NCOLS}
              max={TOP_MAX_NCOLS}
              step={10}
              placeholder={t("top.ncolsPlaceholder")}
              presets={[
                { value: "", label: t("ui.default") },
                { value: "120", label: "120" },
                { value: "200", label: "200" },
              ]}
            />
          </OptionRow>
          <p className="note">{t("top.note")}</p>
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
          emptyHint={t("top.empty")}
        >
          {runner.result ? (
            <TopResultView result={runner.result} />
          ) : null}
        </OutputPane>
      }
    />
  );
}
