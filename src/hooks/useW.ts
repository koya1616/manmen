import { api } from "../api";
import type { WSnapshot } from "../types";
import { useSimpleCommand } from "./useSimpleCommand";

export function useW() {
  return useSimpleCommand<WSnapshot>("w", () => api.getW());
}
