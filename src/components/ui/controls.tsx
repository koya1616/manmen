import { useState, type ReactNode } from "react";
import { useTranslation } from "react-i18next";
import { useLink } from "./Workbench";

// 1つのオプション。data-opt でコマンドバーのトークンと結びつく。
export function OptionRow({
  id,
  label,
  flag,
  hint,
  on = false,
  error,
  children,
}: {
  id: string;
  label: string;
  flag?: string;
  hint?: ReactNode;
  on?: boolean;
  error?: string | null;
  children?: ReactNode;
}) {
  const { linked, flash, setLinked } = useLink();
  return (
    <section
      data-opt={id}
      className={[
        "opt",
        on ? "is-on" : "",
        linked === id ? "is-linked" : "",
        flash === id ? "is-flash" : "",
        error ? "is-invalid" : "",
      ].join(" ")}
      onMouseEnter={() => setLinked(id)}
      onMouseLeave={() => setLinked(null)}
    >
      <div className="opt-head">
        <span className="opt-label">{label}</span>
        {flag ? <code className="opt-flag">{flag}</code> : null}
      </div>
      {children}
      {error ? <p className="opt-error">{error}</p> : null}
      {hint ? <p className="opt-hint">{hint}</p> : null}
    </section>
  );
}

// ON/OFF だけのオプションは行全体をスイッチにする。
export function FlagRow({
  id,
  label,
  flag,
  hint,
  checked,
  onChange,
}: {
  id: string;
  label: string;
  flag: string;
  hint?: string;
  checked: boolean;
  onChange: (next: boolean) => void;
}) {
  const { linked, flash, setLinked } = useLink();
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      data-opt={id}
      className={[
        "opt opt-flagrow",
        checked ? "is-on" : "",
        linked === id ? "is-linked" : "",
        flash === id ? "is-flash" : "",
      ].join(" ")}
      onMouseEnter={() => setLinked(id)}
      onMouseLeave={() => setLinked(null)}
      onClick={() => onChange(!checked)}
    >
      <span className="opt-flagrow-copy">
        <span className="opt-head">
          <span className="opt-label">{label}</span>
          <code className="opt-flag">{flag}</code>
        </span>
        {hint ? <span className="opt-hint">{hint}</span> : null}
      </span>
      <span className={`toggle ${checked ? "is-on" : ""}`} aria-hidden>
        <span className="toggle-knob" />
      </span>
    </button>
  );
}

export function Segmented<T extends string>({
  value,
  options,
  onChange,
  wrap = false,
}: {
  value: T;
  options: { value: T; label: ReactNode; sub?: ReactNode }[];
  onChange: (next: T) => void;
  wrap?: boolean;
}) {
  return (
    <div className={`seg ${wrap ? "seg-wrap" : ""}`} role="radiogroup">
      {options.map((option) => (
        <button
          key={option.value || "__empty"}
          type="button"
          role="radio"
          aria-checked={option.value === value}
          className={option.value === value ? "is-selected" : ""}
          onClick={() => onChange(option.value)}
        >
          <span className="seg-label">{option.label}</span>
          {option.sub ? <span className="seg-sub">{option.sub}</span> : null}
        </button>
      ))}
    </div>
  );
}

export function Toggle({
  checked,
  onChange,
  onLabel = "ON",
  offLabel = "OFF",
}: {
  checked: boolean;
  onChange: (next: boolean) => void;
  onLabel?: string;
  offLabel?: string;
}) {
  return (
    <button
      type="button"
      role="switch"
      aria-checked={checked}
      className="toggle-big"
      onClick={() => onChange(!checked)}
    >
      <span className={`toggle ${checked ? "is-on" : ""}`} aria-hidden>
        <span className="toggle-knob" />
      </span>
      <span className="toggle-text">{checked ? onLabel : offLabel}</span>
    </button>
  );
}

// 数値入力。± ボタンと、よく使う値のプリセットを並べる。
export function Stepper({
  value,
  onChange,
  min,
  max,
  step = 1,
  unit,
  placeholder,
  presets = [],
}: {
  value: string;
  onChange: (next: string) => void;
  min: number;
  max: number;
  step?: number;
  unit?: string;
  placeholder?: string;
  presets?: { value: string; label: string }[];
}) {
  const n = Number(value);
  const base = value.trim() === "" || !Number.isFinite(n) ? min : n;

  function bump(delta: number) {
    const next = Math.min(max, Math.max(min, base + delta));
    onChange(String(next));
  }

  return (
    <div className="stepper-wrap">
      <div className="stepper">
        <button type="button" onClick={() => bump(-step)} aria-label="-">
          −
        </button>
        <input
          type="text"
          inputMode="numeric"
          value={value}
          placeholder={placeholder}
          onChange={(e) => onChange(e.target.value.replace(/[^\d]/g, ""))}
          onKeyDown={(e) => {
            if (e.key === "ArrowUp") {
              e.preventDefault();
              bump(e.shiftKey ? step * 10 : step);
            } else if (e.key === "ArrowDown") {
              e.preventDefault();
              bump(e.shiftKey ? -step * 10 : -step);
            }
          }}
        />
        {unit ? <span className="stepper-unit">{unit}</span> : null}
        <button type="button" onClick={() => bump(step)} aria-label="+">
          +
        </button>
      </div>
      {presets.length > 0 ? (
        <div className="presets">
          {presets.map((preset) => (
            <button
              key={preset.value}
              type="button"
              className={preset.value === value ? "is-selected" : ""}
              onClick={() => onChange(preset.value)}
            >
              {preset.label}
            </button>
          ))}
        </div>
      ) : null}
    </div>
  );
}

export function TextField({
  value,
  onChange,
  placeholder,
  invalid = false,
  onClear,
}: {
  value: string;
  onChange: (next: string) => void;
  placeholder?: string;
  invalid?: boolean;
  onClear?: () => void;
}) {
  return (
    <div className={`textfield ${invalid ? "is-invalid" : ""}`}>
      <input
        type="text"
        value={value}
        placeholder={placeholder}
        spellCheck={false}
        autoCorrect="off"
        autoCapitalize="off"
        onChange={(e) => onChange(e.target.value)}
      />
      {value && onClear ? (
        <button type="button" className="textfield-clear" onClick={onClear} aria-label="clear">
          ×
        </button>
      ) : null}
    </div>
  );
}

// キーの一覧からチップで選ぶ。よく使うキーだけ先に見せ、残りは展開して検索する。
// multi のときは選んだ順に番号を出す (top -stats は順番どおりに列が並ぶため)。
export function KeyPicker({
  keys,
  featured,
  selected,
  onPick,
  labelOf,
  helpOf,
  multi = false,
}: {
  keys: readonly string[];
  featured: readonly string[];
  selected: string[];
  onPick: (key: string) => void;
  labelOf: (key: string) => string;
  helpOf: (key: string) => string;
  multi?: boolean;
}) {
  const { t } = useTranslation();
  const [expanded, setExpanded] = useState(false);
  const [query, setQuery] = useState("");
  const [hover, setHover] = useState<string | null>(null);

  const q = query.trim().toLowerCase();
  const visible = expanded
    ? keys.filter(
        (key) => !q || key.includes(q) || labelOf(key).toLowerCase().includes(q),
      )
    : [...featured, ...selected.filter((key) => !featured.includes(key))];
  const focus = hover ?? selected[selected.length - 1] ?? null;

  return (
    <div className="keypicker">
      {expanded ? (
        <input
          className="keypicker-search"
          type="text"
          value={query}
          autoFocus
          placeholder={t("ui.searchKeys")}
          onChange={(e) => setQuery(e.target.value)}
        />
      ) : null}
      <div className="chips">
        {visible.map((key) => {
          const order = selected.indexOf(key);
          return (
            <button
              key={key}
              type="button"
              className={`chip ${order >= 0 ? "is-selected" : ""}`}
              onClick={() => onPick(key)}
              onMouseEnter={() => setHover(key)}
              onMouseLeave={() => setHover(null)}
            >
              {multi && order >= 0 ? <span className="chip-order">{order + 1}</span> : null}
              <span className="chip-label">{labelOf(key)}</span>
              <code className="chip-key">{key}</code>
            </button>
          );
        })}
        {visible.length === 0 ? <span className="muted">{t("ui.noMatch")}</span> : null}
        <button
          type="button"
          className="chip chip-more"
          onClick={() => {
            setExpanded(!expanded);
            setQuery("");
          }}
        >
          {expanded ? t("ui.showLess") : t("ui.showAll", { n: keys.length })}
        </button>
      </div>
      <p className="opt-hint keypicker-help">
        {focus ? (
          <>
            <code>{focus}</code> {helpOf(focus)}
          </>
        ) : (
          " "
        )}
      </p>
    </div>
  );
}
