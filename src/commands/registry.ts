import type { ComponentType } from "react";
import { DisablesleepAbout } from "../components/DisablesleepAbout";
import { DisablesleepCard } from "../components/DisablesleepCard";

// 新しいコマンドの追加手順:
// 1. hooks/ と components/ に対応UIを追加する (useDisablesleep.ts 等を参照)
// 2. ここにエントリを1行追加する
export interface CommandEntry {
  id: string;
  control: ComponentType;
  about: ComponentType;
}

export const commandRegistry: CommandEntry[] = [
  {
    id: "pmset.disablesleep",
    control: DisablesleepCard,
    about: DisablesleepAbout,
  },
];
