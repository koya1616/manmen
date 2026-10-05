import { useEffect, useRef, useState, type KeyboardEvent } from "react";
import { useTranslation } from "react-i18next";
import type { CommandEntry } from "../commands/registry";

// ⌘K で開くコマンド検索。コマンド名と日英両方の表示名で部分一致検索する。
// ↑↓ で選択、Enter で切り替え、Esc か背景クリックで閉じる。
export function CommandPalette({
  entries,
  open,
  onClose,
  onSelect,
}: {
  entries: CommandEntry[];
  open: boolean;
  onClose: () => void;
  onSelect: (id: string) => void;
}) {
  const { t, i18n } = useTranslation();
  const [query, setQuery] = useState("");
  const [cursor, setCursor] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);
  const listRef = useRef<HTMLUListElement>(null);

  useEffect(() => {
    if (!open) return;
    setQuery("");
    setCursor(0);
    inputRef.current?.focus();
  }, [open]);

  const needle = query.trim().toLowerCase();
  const matches = entries
    .map((entry, index) => ({ entry, index, rank: rankOf(entry, needle) }))
    .filter((m) => m.rank !== null)
    .sort((a, b) => (a.rank ?? 0) - (b.rank ?? 0) || a.index - b.index);

  function rankOf(entry: CommandEntry, q: string): number | null {
    if (q === "") return 0;
    const cmd = entry.cmd.toLowerCase();
    if (cmd.startsWith(q)) return 0;
    if (cmd.includes(q)) return 1;
    const nameKey = entry.nameKey.replace(/\.label$/, ".name");
    const texts = [
      t(entry.nameKey),
      t(nameKey),
      i18n.t(entry.nameKey, { lng: "en" }),
      i18n.t(entry.nameKey, { lng: "ja" }),
    ].map((text) => text.toLowerCase());
    return texts.some((text) => text.includes(q)) ? 2 : null;
  }

  useEffect(() => {
    listRef.current
      ?.querySelector<HTMLElement>(`[data-index="${cursor}"]`)
      ?.scrollIntoView({ block: "nearest" });
  }, [cursor]);

  if (!open) return null;

  function choose(id: string) {
    onSelect(id);
    onClose();
  }

  function onKeyDown(e: KeyboardEvent) {
    if (e.key === "ArrowDown") {
      e.preventDefault();
      setCursor((c) => (matches.length === 0 ? 0 : (c + 1) % matches.length));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      setCursor((c) => (matches.length === 0 ? 0 : (c - 1 + matches.length) % matches.length));
    } else if (e.key === "Enter" && !e.nativeEvent.isComposing) {
      e.preventDefault();
      const hit = matches[cursor];
      if (hit) choose(hit.entry.id);
    } else if (e.key === "Escape") {
      e.preventDefault();
      onClose();
    }
  }

  return (
    <div className="palette-backdrop" onMouseDown={onClose}>
      <div
        className="palette"
        role="dialog"
        aria-label={t("palette.title")}
        onMouseDown={(e) => e.stopPropagation()}
      >
        <input
          ref={inputRef}
          className="palette-input"
          value={query}
          placeholder={t("palette.placeholder")}
          onChange={(e) => {
            setQuery(e.target.value);
            setCursor(0);
          }}
          onKeyDown={onKeyDown}
          spellCheck={false}
          autoComplete="off"
        />
        {matches.length === 0 ? (
          <p className="palette-empty">{t("palette.noMatch")}</p>
        ) : (
          <ul className="palette-list" ref={listRef}>
            {matches.map((m, i) => (
              <li key={m.entry.id}>
                <button
                  type="button"
                  data-index={i}
                  className={i === cursor ? "is-cursor" : ""}
                  onMouseEnter={() => setCursor(i)}
                  onClick={() => choose(m.entry.id)}
                >
                  <code className="nav-cmd">{m.entry.cmd}</code>
                  <span className="nav-label">{t(m.entry.nameKey)}</span>
                  {m.index < 9 ? <kbd className="nav-key">⌘{m.index + 1}</kbd> : null}
                </button>
              </li>
            ))}
          </ul>
        )}
        <p className="palette-foot">{t("palette.help")}</p>
      </div>
    </div>
  );
}
