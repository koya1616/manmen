import { useTranslation } from "react-i18next";
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
  const { t } = useTranslation();

  const cmdName = t(`commands.${command.id}.name`, command.name);
  const cmdDesc = t(`commands.${command.id}.description`, command.description);

  const translatedArgs = command.arguments.map((arg) => ({
    ...arg,
    name: t(`commands.${command.id}.arguments.${arg.id}.name`, arg.name),
    description: t(`commands.${command.id}.arguments.${arg.id}.description`, arg.description),
  }));

  return (
    <div className="command-detail">
      <header className="command-header">
        <h1>{cmdName}</h1>
        <p className="command-description">{cmdDesc}</p>
        <div className="command-meta">
          <span className="category">{t(`categories.${command.category}`, command.category)}</span>
          {command.requires_admin && (
            <span className="badge admin">{t("command.adminRequired")}</span>
          )}
          {command.dangerous && (
            <span className="badge dangerous">{t("command.dangerous")}</span>
          )}
        </div>
      </header>

      {command.requires_admin && (
        <div className="warning-banner">
          <p>{t("command.adminWarning")}</p>
        </div>
      )}

      <section className="arguments-section">
        <h2>{t("command.arguments")}</h2>
        {translatedArgs.length > 0 ? (
          <DynamicForm
            arguments={translatedArgs}
            values={args}
            onChange={onArgChange}
            errors={validation?.errors?.reduce(
              (acc, err) => ({ ...acc, [err.field]: err.message }),
              {} as Record<string, string>
            )}
          />
        ) : (
          <p className="no-arguments">{t("command.noArguments")}</p>
        )}
      </section>

      <section className="preview-section">
        <h2>{t("command.generatedCommand")}</h2>
        <div className="command-preview">
          <code>{preview?.full_command || t("command.configureAbove")}</code>
        </div>
        <div className="preview-actions">
          <button onClick={onValidate} className="btn-secondary">
            {t("command.validate")}
          </button>
          <button onClick={onBuild} className="btn-secondary">
            {t("command.preview")}
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
          {executing ? t("command.executing") : t("command.execute")}
        </button>
      </section>
    </div>
  );
}
