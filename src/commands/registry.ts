import type { ComponentType } from "react";
import { PmsetAbout } from "../components/PmsetAbout";
import { PmsetCard } from "../components/PmsetCard";
import { ManpageAbout } from "../components/ManpageAbout";
import { ManpageCard } from "../components/ManpageCard";
import { TopAbout } from "../components/TopAbout";
import { TopCard } from "../components/TopCard";

// 新しいコマンドの追加手順:
// 1. hooks/ と components/ に対応UIを追加する (usePmset.ts 等を参照)
// 2. ここにエントリを1行追加する
export interface CommandEntry {
  id: string;
  nameKey: string;
  control: ComponentType;
  about: ComponentType;
}

export const commandRegistry: CommandEntry[] = [
  {
    id: "pmset.apply",
    nameKey: "commands.pmset.name",
    control: PmsetCard,
    about: PmsetAbout,
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
