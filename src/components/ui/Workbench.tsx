import {
  createContext,
  useContext,
  useEffect,
  useRef,
  useState,
  type ReactNode,
} from "react";
import { useTranslation } from "react-i18next";
import { tokensToString, type CmdToken } from "../../commands/tokens";

// オプション行とコマンドバーのトークンを結ぶ。
// 片方にカーソルを乗せるともう片方も光り、トークンをクリックすると該当行へ飛ぶ。
interface LinkState {
  linked: string | null;
  flash: string | null;
  setLinked: (id: string | null) => void;
}

const LinkContext = createContext<LinkState>({
  linked: null,
  flash: null,
  setLinked: () => undefined,
});

export function useLink() {
  return useContext(LinkContext);
}

function CommandBar({
  tokens,
  onJump,
}: {
  tokens: CmdToken[];
  onJump: (opt: string) => void;
}) {
  const { linked, setLinked } = useLink();

  return (
    <code className="cmd-line" aria-live="polite">
      <span className="cmd-prompt">$</span>
      {tokens.map((token, index) => (
        <span
          key={`${index}-${token.text}`}
          className={[
            "tk",
            `tk-${token.kind}`,
            token.opt ? "tk-link" : "",
            token.opt && token.opt === linked ? "is-linked" : "",
          ].join(" ")}
          onMouseEnter={() => token.opt && setLinked(token.opt)}
          onMouseLeave={() => token.opt && setLinked(null)}
          onClick={() => token.opt && onJump(token.opt)}
        >
          {token.text}
        </span>
      ))}
    </code>
  );
}

export function Workbench({
  active,
  title,
  description,
  tokens,
  onRun,
  canRun,
  running,
  runLabel,
  runningLabel,
  danger = false,
  blocker,
  notice,
  options,
  output,
  about,
}: {
  active: boolean;
  title: string;
  description: string;
  tokens: CmdToken[];
  onRun: () => void;
  canRun: boolean;
  running: boolean;
  runLabel: string;
  runningLabel: string;
  danger?: boolean;
  blocker?: string | null;
  notice?: ReactNode;
  options: ReactNode;
  output: ReactNode;
  about: ReactNode;
}) {
  const { t } = useTranslation();
  const [linked, setLinked] = useState<string | null>(null);
  const [flash, setFlash] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);
  const optionsRef = useRef<HTMLDivElement>(null);
  const runRef = useRef(onRun);
  runRef.current = onRun;
  const ready = canRun && !running;

  useEffect(() => {
    if (!active) return;
    function onKey(e: KeyboardEvent) {
      if (e.key === "Enter" && (e.metaKey || e.ctrlKey) && ready) {
        e.preventDefault();
        runRef.current();
      }
    }
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [active, ready]);

  function jump(opt: string) {
    const el = optionsRef.current?.querySelector(`[data-opt="${opt}"]`);
    el?.scrollIntoView({ block: "center", behavior: "smooth" });
    setFlash(opt);
    window.setTimeout(() => setFlash((cur) => (cur === opt ? null : cur)), 1200);
  }

  async function copy() {
    try {
      await navigator.clipboard.writeText(tokensToString(tokens));
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1200);
    } catch {
      // クリップボードが使えない環境では何もしない
    }
  }

  return (
    <LinkContext.Provider value={{ linked, flash, setLinked }}>
      <div className="wb" hidden={!active}>
        <header className="wb-head">
          <div className="wb-title">
            <h2>{title}</h2>
            <p>{description}</p>
          </div>
        </header>

        <div className={`wb-cmd ${danger ? "is-danger" : ""}`}>
          <CommandBar tokens={tokens} onJump={jump} />
          <div className="wb-cmd-actions">
            <button type="button" className="btn-ghost" onClick={copy}>
              {copied ? t("ui.copied") : t("ui.copy")}
            </button>
            <button
              type="button"
              className={`btn-run ${danger ? "is-danger" : ""}`}
              onClick={onRun}
              disabled={!ready}
              title={blocker ?? undefined}
            >
              {running ? <span className="spinner" aria-hidden /> : <span aria-hidden>▶</span>}
              <span>{running ? runningLabel : runLabel}</span>
              {!running ? <kbd>⌘↵</kbd> : null}
            </button>
          </div>
        </div>
        {blocker ? <p className="wb-blocker">{blocker}</p> : null}
        {notice ? <div className="wb-notice">{notice}</div> : null}

        <div className="wb-body">
          <div className="wb-options" ref={optionsRef}>
            <div className="pane-label">{t("ui.options")}</div>
            {options}
            <details className="about">
              <summary>{t("ui.about")}</summary>
              {about}
            </details>
          </div>
          <div className="wb-output">{output}</div>
        </div>
      </div>
    </LinkContext.Provider>
  );
}
