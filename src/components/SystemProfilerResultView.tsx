import { useTranslation } from "react-i18next";
import type { ProfilerValue } from "../types";

function Scalar({ value }: { value: string | number | boolean | null }) {
  if (value === null) return <span className="jtree-null">null</span>;
  if (typeof value === "string") return <span className="jtree-str">{value}</span>;
  if (typeof value === "boolean") return <span className="jtree-bool">{String(value)}</span>;
  return <span className="jtree-num">{String(value)}</span>;
}

// system_profiler の JSON を折りたたみツリーで表示する。
// 型ごとにスキーマが違うためキー列挙の表にはしない。浅い階層は開いておく。
function JsonNode({ value, depth }: { value: ProfilerValue; depth: number }) {
  if (value === null || typeof value !== "object") {
    return <Scalar value={value} />;
  }
  if (Array.isArray(value)) {
    if (value.length === 0) return <span className="jtree-empty">[]</span>;
    return (
      <details className="jtree" open={depth < 2}>
        <summary className="jtree-summary">[{value.length}]</summary>
        <ul className="jtree-list">
          {value.map((item, index) => (
            <li key={index} className="jtree-item">
              <span className="jtree-index">{index}</span>
              <JsonNode value={item} depth={depth + 1} />
            </li>
          ))}
        </ul>
      </details>
    );
  }
  const entries = Object.entries(value);
  if (entries.length === 0) return <span className="jtree-empty">{"{}"}</span>;
  return (
    <details className="jtree" open={depth < 2}>
      <summary className="jtree-summary">{"{"}{entries.length}{"}"}</summary>
      <dl className="jtree-list">
        {entries.map(([key, item]) => (
          <div key={key} className="jtree-item">
            <dt className="jtree-key">{key}</dt>
            <dd className="jtree-val">
              <JsonNode value={item} depth={depth + 1} />
            </dd>
          </div>
        ))}
      </dl>
    </details>
  );
}

export function SystemProfilerResultView({ value }: { value: ProfilerValue }) {
  const { t } = useTranslation();
  if (value === null) {
    return <p className="muted">{t("spResult.noData")}</p>;
  }
  return (
    <div className="jtree-root mono">
      <JsonNode value={value} depth={0} />
    </div>
  );
}
