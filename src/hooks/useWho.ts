import { api } from "../api";
import type { WhoSnapshot } from "../types";
import { useSimpleCommand } from "./useSimpleCommand";

export function useWho() {
  return useSimpleCommand<WhoSnapshot>("who", () => api.getWho());
}
