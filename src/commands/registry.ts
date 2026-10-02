import type { ComponentType, ReactNode } from "react";
import { PmsetAbout } from "../components/PmsetAbout";
import { PmsetCard } from "../components/PmsetCard";
import { ManpageAbout } from "../components/ManpageAbout";
import { ManpageCard } from "../components/ManpageCard";
import { TopAbout } from "../components/TopAbout";
import { TopCard } from "../components/TopCard";
import { DockerAbout } from "../components/DockerAbout";
import { DockerCard } from "../components/DockerCard";

// 新しいコマンドの追加手順:
// 1. hooks/ と components/ に対応UIを追加する (usePmset.ts 等を参照)
// 2. ここにエントリを1行追加する
export interface CommandEntry {
  id: string;
  // サイドバーに等幅で出すコマンド名
  cmd: string;
  nameKey: string;
  control: ComponentType<{ active: boolean; about: ReactNode }>;
  about: ComponentType;
}

export const commandRegistry: CommandEntry[] = [
  {
    id: "pmset.apply",
    cmd: "pmset",
    nameKey: "commands.pmset.label",
    control: PmsetCard,
    about: PmsetAbout,
  },
  {
    id: "man.manpage",
    cmd: "man",
    nameKey: "commands.man.label",
    control: ManpageCard,
    about: ManpageAbout,
  },
  {
    id: "top.process",
    cmd: "top",
    nameKey: "commands.top.label",
    control: TopCard,
    about: TopAbout,
  },
  {
    id: "docker.builderPrune",
    cmd: "docker",
    nameKey: "commands.docker.label",
    control: DockerCard,
    about: DockerAbout,
  },
];
