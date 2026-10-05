import { api } from "../api";
import type { CommandResult } from "../types";
import { useSimpleCommand } from "./useSimpleCommand";

export function useWhoami() {
  return useSimpleCommand<CommandResult>("whoami", () => api.getWhoami());
}
