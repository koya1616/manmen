import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { MAN_TOPIC_GROUPS } from "../commands/manTopics";
import { useManpage } from "../hooks/useManpage";
import { ManpageResultView } from "./ManpageResultView";

const VALID_TOPICS = new Set(
  MAN_TOPIC_GROUPS.flatMap((group) => group.topics),
);

export function ManpageCard() {
  const { t } = useTranslation();
  const { topic, setTopic, executing, result, error, preview, execute } =
    useManpage();

  const filteredGroups = useMemo(() => {
    const q = topic.trim().toLowerCase();
    if (!q) return MAN_TOPIC_GROUPS;
    return MAN_TOPIC_GROUPS.map((group) => ({
      ...group,
      topics: group.topics.filter(
        (name) =>
          name.toLowerCase().includes(q) ||
          t(`manTopics.${name}`).toLowerCase().includes(q),
      ),
    })).filter((group) => group.topics.length > 0);
  }, [topic, t]);

  const flatMatches = filteredGroups.flatMap((group) => group.topics);
  const isValid = VALID_TOPICS.has(topic.trim());
  // 完全一致1件だけなら選び直す必要がないので候補を隠す
  const showSuggest =
    flatMatches.length > 0 &&
    !(flatMatches.length === 1 && flatMatches[0] === topic.trim());

  function handleKeyDown(e: React.KeyboardEvent) {
    if (e.key === "Enter" && isValid && !executing) {
      execute();
    }
  }

  return (
    <>
      <section className="card">
        <h2>{t("man.title")}</h2>
        <p className="muted">{t("man.description")}</p>

        <input
          type="text"
          className="search-input"
          value={topic}
          onChange={(e) => setTopic(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={t("man.searchPlaceholder")}
          aria-label={t("man.topic")}
        />

        {showSuggest && (
          <ul className="suggest-list">
            {filteredGroups.map((group) => (
              <li key={group.labelKey}>
                <span className="suggest-group">{t(group.labelKey)}</span>
                <ul>
                  {group.topics.map((name) => (
                    <li key={name}>
                      <button onClick={() => setTopic(name)}>
                        <code>{name}</code>
                        <span>{t(`manTopics.${name}`)}</span>
                      </button>
                    </li>
                  ))}
                </ul>
              </li>
            ))}
          </ul>
        )}
        {topic.trim() !== "" && flatMatches.length === 0 && (
          <p className="muted">{t("man.noMatch")}</p>
        )}

        <div className="topic-row">
          <button
            className="btn-primary"
            onClick={execute}
            disabled={executing || !isValid}
          >
            {executing ? t("man.executing") : t("man.execute")}
          </button>
        </div>

        <div className="preview">
          <span className="preview-label">{t("man.preview")}</span>
          <code>{preview}</code>
        </div>
      </section>

      <ManpageResultView result={result} error={error} />
    </>
  );
}
