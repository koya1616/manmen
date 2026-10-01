import type { ReactNode } from "react";
import { useTranslation } from "react-i18next";
import type { ManInline, ManpageDocument } from "../types";

const OPEN_SECTIONS = new Set([
  "NAME",
  "SYNOPSIS",
  "DESCRIPTION",
  "EXAMPLES",
]);

function openSection(id: string) {
  const el = document.getElementById(id);
  if (el instanceof HTMLDetailsElement) el.open = true;
  el?.scrollIntoView({ block: "start" });
}

function renderInlines(spans: ManInline[]): ReactNode[] {
  return spans.map((span, index) => {
    const key = `${span.kind}-${index}`;
    if (span.kind === "bold") return <strong key={key}>{renderInlines(span.children)}</strong>;
    if (span.kind === "italic") return <em key={key}>{renderInlines(span.children)}</em>;
    if (span.kind === "code") return <code key={key}>{span.text}</code>;
    return <span key={key}>{span.text}</span>;
  });
}

// man のセクション表示。生テキストではなく見出しごとの折りたたみで見せる。
export function ManpageResultView({
  result,
  error,
}: {
  result: ManpageDocument | null;
  error: string | null;
}) {
  const { t } = useTranslation();

  if (error) {
    return (
      <section className="card error">
        <h3>{t("result.failed")}</h3>
        <pre>{error}</pre>
      </section>
    );
  }

  if (!result) return null;

  const empty = result.sections.length === 0;

  return (
    <section className={result.success ? "card" : "card error"}>
      <h3>{result.success ? t("result.completed") : t("result.failed")}</h3>
      {result.title ? <p className="man-title">{result.title}</p> : null}

      {empty ? (
        <p className="muted">{result.stderr || t("manResult.noContent")}</p>
      ) : (
        <>
          <nav className="man-toc">
            {result.sections.map((section, index) => (
              <button
                key={section.title + index}
                type="button"
                onClick={() => openSection(`man-section-${index}`)}
              >
                {section.title}
              </button>
            ))}
          </nav>
          <div className="man-sections">
            {result.sections.map((section, index) => (
              <details
                key={section.title + index}
                id={`man-section-${index}`}
                className="man-section"
                open={OPEN_SECTIONS.has(section.title)}
              >
                <summary>{section.title}</summary>
                {section.blocks.map((block, blockIndex) => (
                  <p
                    key={blockIndex}
                    className={block.kind === "quote" ? "man-quote" : "man-block"}
                  >
                    {renderInlines(block.inlines)}
                  </p>
                ))}
              </details>
            ))}
          </div>
        </>
      )}

      {result.stderr && !empty ? (
        <div className="output-block">
          <h4>{t("result.error")}</h4>
          <pre>{result.stderr}</pre>
        </div>
      ) : null}

      <details className="raw-details">
        <summary>{t("manResult.raw")}</summary>
        <p className="muted">
          {t("result.exitCode")}: {result.exit_code}
        </p>
        <p className="muted">
          <code>{result.command}</code>
        </p>
      </details>
    </section>
  );
}
