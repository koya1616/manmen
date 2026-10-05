import type { ComponentType, ReactNode } from "react";
import { PmsetAbout } from "../components/PmsetAbout";
import { PmsetCard } from "../components/PmsetCard";
import { ManpageAbout } from "../components/ManpageAbout";
import { ManpageCard } from "../components/ManpageCard";
import { DigAbout } from "../components/DigAbout";
import { DigCard } from "../components/DigCard";
import { SshAbout } from "../components/SshAbout";
import { SshCard } from "../components/SshCard";
import { TopAbout } from "../components/TopAbout";
import { TopCard } from "../components/TopCard";
import { PsAbout } from "../components/PsAbout";
import { PsCard } from "../components/PsCard";
import { LsofAbout } from "../components/LsofAbout";
import { LsofCard } from "../components/LsofCard";
import { DockerAbout } from "../components/DockerAbout";
import { DockerCard } from "../components/DockerCard";
import { WhoamiAbout } from "../components/WhoamiAbout";
import { WhoamiCard } from "../components/WhoamiCard";
import { WhoAbout } from "../components/WhoAbout";
import { WhoCard } from "../components/WhoCard";
import { WAbout } from "../components/WAbout";
import { WCard } from "../components/WCard";
import { SystemProfilerAbout } from "../components/SystemProfilerAbout";
import { SystemProfilerCard } from "../components/SystemProfilerCard";
import { ScutilAbout } from "../components/ScutilAbout";
import { ScutilCard } from "../components/ScutilCard";
import { GitAbout } from "../components/GitAbout";
import { GitCard } from "../components/GitCard";
import { PingAbout } from "../components/PingAbout";
import { PingCard } from "../components/PingCard";
import { CurlAbout } from "../components/CurlAbout";
import { CurlCard } from "../components/CurlCard";
import { KillAbout } from "../components/KillAbout";
import { KillCard } from "../components/KillCard";

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
    id: "dig.lookup",
    cmd: "dig",
    nameKey: "commands.dig.label",
    control: DigCard,
    about: DigAbout,
  },
  {
    id: "ssh.connect",
    cmd: "ssh",
    nameKey: "commands.ssh.label",
    control: SshCard,
    about: SshAbout,
  },
  {
    id: "top.process",
    cmd: "top",
    nameKey: "commands.top.label",
    control: TopCard,
    about: TopAbout,
  },
  {
    id: "ps.snapshot",
    cmd: "ps",
    nameKey: "commands.ps.label",
    control: PsCard,
    about: PsAbout,
  },
  {
    id: "lsof.files",
    cmd: "lsof",
    nameKey: "commands.lsof.label",
    control: LsofCard,
    about: LsofAbout,
  },
  {
    id: "docker.builderPrune",
    cmd: "docker",
    nameKey: "commands.docker.label",
    control: DockerCard,
    about: DockerAbout,
  },
  {
    id: "whoami.user",
    cmd: "whoami",
    nameKey: "commands.whoami.label",
    control: WhoamiCard,
    about: WhoamiAbout,
  },
  {
    id: "who.users",
    cmd: "who",
    nameKey: "commands.who.label",
    control: WhoCard,
    about: WhoAbout,
  },
  {
    id: "w.users",
    cmd: "w",
    nameKey: "commands.w.label",
    control: WCard,
    about: WAbout,
  },
  {
    id: "system_profiler.overview",
    cmd: "system_profiler",
    nameKey: "commands.system_profiler.label",
    control: SystemProfilerCard,
    about: SystemProfilerAbout,
  },
  {
    id: "scutil.network",
    cmd: "scutil",
    nameKey: "commands.scutil.label",
    control: ScutilCard,
    about: ScutilAbout,
  },
  {
    id: "git.repo",
    cmd: "git",
    nameKey: "commands.git.label",
    control: GitCard,
    about: GitAbout,
  },
  {
    id: "ping.probe",
    cmd: "ping",
    nameKey: "commands.ping.label",
    control: PingCard,
    about: PingAbout,
  },
  {
    id: "curl.request",
    cmd: "curl",
    nameKey: "commands.curl.label",
    control: CurlCard,
    about: CurlAbout,
  },
  {
    id: "kill.signal",
    cmd: "kill",
    nameKey: "commands.kill.label",
    control: KillCard,
    about: KillAbout,
  },
];
