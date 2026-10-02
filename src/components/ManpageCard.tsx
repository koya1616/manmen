import { useMemo, useState, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { MAN_TOPIC_GROUPS } from "../commands/manTopics";
import { useManpage } from "../hooks/useManpage";
import { ManpageResultView } from "./ManpageResultView";
import { OutputPane } from "./ui/OutputPane";
import { useLink, Workbench } from "./ui/Workbench";

const VALID_TOPICS = new Set(MAN_TOPIC_GROUPS.flatMap((group) => group.topics));

// トピック一覧。クリック / Enter でそのまま表示する。↑↓ で候補を移動できる。
function TopicList({
  topic,
  current,
  onPick,
}: {
  topic: string;
  current: string | null;
  onPick: (name: string) => void;
}) {
  const { t } = useTranslation();
  const { linked, flash, setLinked } = useLink();
  const [query, setQuery] = useState("");
  const [group, setGroup] = useState<string | null>(null);
  const [cursor, setCursor] = useState(0);

  const groups = useMemo(() => {
    const q = query.trim().toLowerCase();
    return MAN_TOPIC_GROUPS.filter((g) => !group || g.labelKey === group)
      .map((g) => ({
        ...g,
        topics: g.topics.filter(
          (name) =>
            !q ||
            name.toLowerCase().includes(q) ||
            t(`manTopics.${name}`).toLowerCase().includes(q),
        ),
      }))
      .filter((g) => g.topics.length > 0);
  }, [query, group, t]);
  const flat = groups.flatMap((g) => g.topics);

  function onKeyDown(e: React.KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setCursor((c) => Math.min(flat.length - 1, c + 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setCursor((c) => Math.max(0, c - 1));
    } else if (e.key === "Enter" && !e.metaKey && flat[cursor]) {
      e.preventDefault();
      onPick(flat[cursor]);
    }
  }

  return (
    <section
      data-opt="topic"
      className={`opt opt-topics ${linked === "topic" ? "is-linked" : ""} ${flash === "topic" ? "is-flash" : ""}`}
      onMouseEnter={() => setLinked("topic")}
      onMouseLeave={() => setLinked(null)}
    >
      <div className="opt-head">
        <span className="opt-label">{t("man.topic")}</span>
        <code className="opt-flag">{topic || "…"}</code>
      </div>
      <input
        className="keypicker-search"
        type="search"
        value={query}
        placeholder={t("man.searchPlaceholder")}
        onChange={(e) => {
          setQuery(e.target.value);
          setCursor(0);
        }}
        onKeyDown={onKeyDown}
      />
      <div className="presets">
        <button
          type="button"
          className={group === null ? "is-selected" : ""}
          onClick={() => setGroup(null)}
        >
          {t("ui.all")}
        </button>
        {MAN_TOPIC_GROUPS.map((g) => (
          <button
            key={g.labelKey}
            type="button"
            className={group === g.labelKey ? "is-selected" : ""}
            onClick={() => {
              setGroup(group === g.labelKey ? null : g.labelKey);
              setCursor(0);
            }}
          >
            {t(g.labelKey)}
          </button>
        ))}
      </div>
      <div className="topic-list">
        {groups.map((g) => (
          <div key={g.labelKey}>
            <div className="topic-group">{t(g.labelKey)}</div>
            {g.topics.map((name) => {
              const index = flat.indexOf(name);
              return (
                <button
                  key={name}
                  type="button"
                  className={[
                    "topic-item",
                    name === current ? "is-selected" : "",
                    index === cursor ? "is-cursor" : "",
                  ].join(" ")}
                  onClick={() => onPick(name)}
                  onMouseEnter={() => setCursor(index)}
                >
                  <code>{name}</code>
                  <span>{t(`manTopics.${name}`)}</span>
                </button>
              );
            })}
          </div>
        ))}
        {flat.length === 0 ? <p className="muted">{t("man.noMatch")}</p> : null}
      </div>
    </section>
  );
}

export function ManpageCard({ active, about }: { active: boolean; about: ReactNode }) {
  const { t } = useTranslation();
  const { topic, runner, tokens, preview, execute } = useManpage();
  const isValid = VALID_TOPICS.has(topic.trim());
  const shown = runner.ranCommand?.replace(/^man /, "") ?? null;

  return (
    <Workbench
      active={active}
      title={t("man.title")}
      description={t("man.description")}
      tokens={tokens}
      onRun={() => execute()}
      canRun={isValid}
      running={runner.running}
      runLabel={t("man.execute")}
      runningLabel={t("man.executing")}
      about={about}
      options={
        <TopicList
          topic={topic}
          current={shown}
          onPick={(name) => {
            if (!runner.running) execute(name);
          }}
        />
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
                  stderr: runner.result.sections.length > 0 ? runner.result.stderr : "",
                }
              : null
          }
          ranAt={runner.ranAt}
          durationMs={runner.durationMs}
          ranCommand={runner.ranCommand}
          stale={runner.ranCommand !== null && runner.ranCommand !== preview}
          emptyHint={t("man.empty")}
        >
          {runner.result ? <ManpageResultView key={runner.ranCommand} result={runner.result} /> : null}
        </OutputPane>
      }
    />
  );
}
