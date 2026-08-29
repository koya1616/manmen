import { useTranslation } from "react-i18next";
import type { CommandExecutionResponse } from "../../types";

interface ExecutionResultProps {
  result: CommandExecutionResponse;
}

export function ExecutionResult({ result }: ExecutionResultProps) {
  const { t } = useTranslation();

  return (
    <div className={`execution-result ${result.success ? "success" : "failure"}`}>
      <header className="result-header">
        <span className={`status-icon ${result.success ? "success" : "failure"}`}>
          {result.success ? "✓" : "✕"}
        </span>
        <h2>{result.success ? t("result.completed") : t("result.failed")}</h2>
      </header>

      <div className="result-details">
        <div className="detail-row">
          <span className="label">{t("result.exitCode")}</span>
          <span className="value">{result.exit_code}</span>
        </div>
        <div className="detail-row">
          <span className="label">{t("result.duration")}</span>
          <span className="value">{result.duration_ms}ms</span>
        </div>
      </div>

      <section className="output-section">
        <h3>{t("result.output")}</h3>
        <pre className="output stdout">
          {result.stdout || t("result.noOutput")}
        </pre>
      </section>

      {result.stderr && (
        <section className="output-section">
          <h3>{t("result.error")}</h3>
          <pre className="output stderr">
            {result.stderr}
          </pre>
        </section>
      )}
    </div>
  );
}
