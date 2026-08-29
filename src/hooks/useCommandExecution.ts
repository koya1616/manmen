import { useState } from "react";
import { tauriService } from "../../services/tauri";
import type {
  CommandDetail,
  CommandValidationResult,
  BuildCommandResponse,
  CommandExecutionResponse,
} from "../../types";

export function useCommandExecution() {
  const [command, setCommand] = useState<CommandDetail | null>(null);
  const [args, setArgs] = useState<Record<string, unknown>>({});
  const [validation, setValidation] = useState<CommandValidationResult | null>(null);
  const [preview, setPreview] = useState<BuildCommandResponse | null>(null);
  const [result, setResult] = useState<CommandExecutionResponse | null>(null);
  const [executing, setExecuting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  async function loadCommand(id: string) {
    try {
      const data = await tauriService.getCommand(id);
      setCommand(data);
      setArgs({});
      setValidation(null);
      setPreview(null);
      setResult(null);
      setError(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to load command");
    }
  }

  function updateArg(key: string, value: unknown) {
    setArgs((prev) => ({ ...prev, [key]: value }));
  }

  async function validate() {
    if (!command) return;

    try {
      const result = await tauriService.validateCommand(command.id, args);
      setValidation(result);
      return result;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Validation failed");
      return null;
    }
  }

  async function build() {
    if (!command) return;

    try {
      const result = await tauriService.buildCommand(command.id, args);
      setPreview(result);
      return result;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Build failed");
      return null;
    }
  }

  async function execute() {
    if (!command) return;

    try {
      setExecuting(true);
      setError(null);
      const response = await tauriService.executeCommand({
        command_id: command.id,
        arguments: args,
      });
      setResult(response);
      return response;
    } catch (err) {
      setError(err instanceof Error ? err.message : "Execution failed");
      return null;
    } finally {
      setExecuting(false);
    }
  }

  return {
    command,
    args,
    validation,
    preview,
    result,
    executing,
    error,
    loadCommand,
    updateArg,
    validate,
    build,
    execute,
  };
}
