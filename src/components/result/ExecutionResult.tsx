import type { CommandExecutionResponse } from "../../types";

interface ExecutionResultProps {
  result: CommandExecutionResponse;
}

export function ExecutionResult({ result }: ExecutionResultProps) {
  return (
    <div className={`execution-result ${result.success ? "success" : "failure"}`}>
      <header className="result-header">
        <span className={`status-icon ${result.success ? "success" : "failure"}`}>
          {result.success ? "✓" : "✕"}
        </span>
        <h2>{result.success ? "Command completed" : "Command failed"}</h2>
      </header>

      <div className="result-details">
        <div className="detail-row">
          <span className="label">Exit Code</span>
          <span className="value">{result.exit_code}</span>
        </div>
        <div className="detail-row">
          <span className="label">Duration</span>
          <span className="value">{result.duration_ms}ms</span>
        </div>
      </div>

      <section className="output-section">
        <h3>Output</h3>
        <pre className="output stdout">
          {result.stdout || "No output"}
        </pre>
      </section>

      {result.stderr && (
        <section className="output-section">
          <h3>Error</h3>
          <pre className="output stderr">
            {result.stderr}
          </pre>
        </section>
      )}
    </div>
  );
}
