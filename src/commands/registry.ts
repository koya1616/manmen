import type { ComponentType } from "react";
import { DisablesleepAbout } from "../components/DisablesleepAbout";
import { DisablesleepCard } from "../components/DisablesleepCard";
import { ManpageAbout } from "../components/ManpageAbout";
import { ManpageCard } from "../components/ManpageCard";
import { TopAbout } from "../components/TopAbout";
import { TopCard } from "../components/TopCard";

// 新しいコマンドの追加手順:
// 1. hooks/ と components/ に対応UIを追加する (useDisablesleep.ts 等を参照)
// 2. ここにエントリを1行追加する
export interface CommandEntry {
  id: string;
  nameKey: string;
  control: ComponentType;
  about: ComponentType;
}

export const commandRegistry: CommandEntry[] = [
  {
    id: "pmset.disablesleep",
    nameKey: "commands.pmset.name",
    control: DisablesleepCard,
    about: DisablesleepAbout,
  },
  {
    id: "man.manpage",
    nameKey: "commands.man.name",
    control: ManpageCard,
    about: ManpageAbout,
  },
  {
    id: "top.process",
    nameKey: "commands.top.name",
    control: TopCard,
    about: TopAbout,
  },
];
