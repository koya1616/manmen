import { invoke } from "@tauri-apps/api/core";
import type {
  CommandSummary,
  CommandDetail,
  CommandCategory,
  CommandValidationResult,
  BuildCommandResponse,
  CommandExecutionRequest,
  CommandExecutionResponse,
  HistoryEntry,
  HistoryDetail,
} from "../../types";

export const tauriService = {
  async getCommands(): Promise<CommandSummary[]> {
    return invoke<CommandSummary[]>("get_commands");
  },

  async getCommand(id: string): Promise<CommandDetail> {
    return invoke<CommandDetail>("get_command", { id });
  },

  async searchCommands(query: string): Promise<CommandSummary[]> {
    return invoke<CommandSummary[]>("search_commands", { query });
  },

  async getCategories(): Promise<CommandCategory[]> {
    return invoke<CommandCategory[]>("get_categories");
  },

  async validateCommand(
    commandId: string,
    args: Record<string, unknown>
  ): Promise<CommandValidationResult> {
    return invoke<CommandValidationResult>("validate_command", {
      commandId,
      arguments: args,
    });
  },

  async buildCommand(
    commandId: string,
    args: Record<string, unknown>
  ): Promise<BuildCommandResponse> {
    return invoke<BuildCommandResponse>("build_command", {
      commandId,
      arguments: args,
    });
  },

  async executeCommand(
    request: CommandExecutionRequest
  ): Promise<CommandExecutionResponse> {
    return invoke<CommandExecutionResponse>("execute_command", { request });
  },

  async getExecutionHistory(limit?: number): Promise<HistoryEntry[]> {
    return invoke<HistoryEntry[]>("get_execution_history", { limit });
  },

  async getExecution(id: string): Promise<HistoryDetail> {
    return invoke<HistoryDetail>("get_execution", { id });
  },
};
