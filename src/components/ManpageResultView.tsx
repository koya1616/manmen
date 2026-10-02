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
// 状態・終了コードは OutputPane 側で出す。
export function ManpageResultView({ result }: { result: ManpageDocument }) {
  const { t } = useTranslation();
  const empty = result.sections.length === 0;

  return (
    <>
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
    </>
  );
}
