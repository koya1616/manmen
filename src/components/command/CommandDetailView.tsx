import { DynamicForm } from "../form/DynamicForm";
import type { CommandDetail, CommandValidationResult, BuildCommandResponse } from "../../types";

interface CommandDetailProps {
  command: CommandDetail;
  args: Record<string, unknown>;
  validation: CommandValidationResult | null;
  preview: BuildCommandResponse | null;
  executing: boolean;
  error: string | null;
  onArgChange: (key: string, value: unknown) => void;
  onValidate: () => void;
  onBuild: () => void;
  onExecute: () => void;
}

export function CommandDetailView({
  command,
  args,
  validation,
  preview,
  executing,
  error,
  onArgChange,
  onValidate,
  onBuild,
  onExecute,
}: CommandDetailProps) {
  return (
    <div className="command-detail">
      <header className="command-header">
        <h1>{command.name}</h1>
        <p className="command-description">{command.description}</p>
        <div className="command-meta">
          <span className="category">{command.category}</span>
          {command.requires_admin && (
            <span className="badge admin">Admin Required</span>
          )}
          {command.dangerous && (
            <span className="badge dangerous">Dangerous</span>
          )}
        </div>
      </header>

      <section className="arguments-section">
        <h2>Arguments</h2>
        {command.arguments.length > 0 ? (
          <DynamicForm
            arguments={command.arguments}
            values={args}
            onChange={onArgChange}
            errors={validation?.errors?.reduce(
              (acc, err) => ({ ...acc, [err.field]: err.message }),
              {} as Record<string, string>
            )}
          />
        ) : (
          <p className="no-arguments">This command has no configurable arguments.</p>
        )}
      </section>

      <section className="preview-section">
        <h2>Generated Command</h2>
        <div className="command-preview">
          <code>{preview?.full_command || "Configure arguments above"}</code>
        </div>
        <div className="preview-actions">
          <button onClick={onValidate} className="btn-secondary">
            Validate
          </button>
          <button onClick={onBuild} className="btn-secondary">
            Preview
          </button>
        </div>
      </section>

      {error && (
        <div className="error-banner">
          <p>{error}</p>
        </div>
      )}

      <section className="execute-section">
        <button
          onClick={onExecute}
          disabled={executing || (validation !== null && !validation.valid)}
          className="btn-primary"
        >
          {executing ? "Executing..." : "Execute"}
        </button>
      </section>
    </div>
  );
}
