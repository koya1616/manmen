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
import { TracerouteAbout } from "../components/TracerouteAbout";
import { TracerouteCard } from "../components/TracerouteCard";
import { DfAbout } from "../components/DfAbout";
import { DfCard } from "../components/DfCard";
import { DuAbout } from "../components/DuAbout";
import { DuCard } from "../components/DuCard";
import { IfconfigAbout } from "../components/IfconfigAbout";
import { IfconfigCard } from "../components/IfconfigCard";
import { NetworksetupAbout } from "../components/NetworksetupAbout";
import { NetworksetupCard } from "../components/NetworksetupCard";

// 新しいコマンドの追加手順:
// 1. hooks/ と components/ に対応UIを追加する (usePmset.ts 等を参照)
// 2. ここにエントリを1行追加する (意味の近いグループの中に置く。先頭9件が ⌘1〜⌘9 になる)
export interface CommandEntry {
  id: string;
  // サイドバーに等幅で出すコマンド名
  cmd: string;
  nameKey: string;
  control: ComponentType<{ active: boolean; about: ReactNode }>;
  about: ComponentType;
}

export const commandRegistry: CommandEntry[] = [
  // マニュアル
  {
    id: "man.manpage",
    cmd: "man",
    nameKey: "commands.man.label",
    control: ManpageCard,
    about: ManpageAbout,
  },
  // プロセス
  {
    id: "ps.snapshot",
    cmd: "ps",
    nameKey: "commands.ps.label",
    control: PsCard,
    about: PsAbout,
  },
  {
    id: "top.process",
    cmd: "top",
    nameKey: "commands.top.label",
    control: TopCard,
    about: TopAbout,
  },
  {
    id: "lsof.files",
    cmd: "lsof",
    nameKey: "commands.lsof.label",
    control: LsofCard,
    about: LsofAbout,
  },
  {
    id: "kill.signal",
    cmd: "kill",
    nameKey: "commands.kill.label",
    control: KillCard,
    about: KillAbout,
  },
  // ネットワーク: 相手への疎通・問い合わせ
  {
    id: "ping.probe",
    cmd: "ping",
    nameKey: "commands.ping.label",
    control: PingCard,
    about: PingAbout,
  },
  {
    id: "traceroute.trace",
    cmd: "traceroute",
    nameKey: "commands.traceroute.label",
    control: TracerouteCard,
    about: TracerouteAbout,
  },
  {
    id: "dig.lookup",
    cmd: "dig",
    nameKey: "commands.dig.label",
    control: DigCard,
    about: DigAbout,
  },
  {
    id: "curl.request",
    cmd: "curl",
    nameKey: "commands.curl.label",
    control: CurlCard,
    about: CurlAbout,
  },
  {
    id: "ssh.connect",
    cmd: "ssh",
    nameKey: "commands.ssh.label",
    control: SshCard,
    about: SshAbout,
  },
  // ネットワーク: この Mac の設定
  {
    id: "ifconfig.show",
    cmd: "ifconfig",
    nameKey: "commands.ifconfig.label",
    control: IfconfigCard,
    about: IfconfigAbout,
  },
  {
    id: "networksetup.read",
    cmd: "networksetup",
    nameKey: "commands.networksetup.label",
    control: NetworksetupCard,
    about: NetworksetupAbout,
  },
  {
    id: "scutil.network",
    cmd: "scutil",
    nameKey: "commands.scutil.label",
    control: ScutilCard,
    about: ScutilAbout,
  },
  // ディスク
  {
    id: "df.usage",
    cmd: "df",
    nameKey: "commands.df.label",
    control: DfCard,
    about: DfAbout,
  },
  {
    id: "du.usage",
    cmd: "du",
    nameKey: "commands.du.label",
    control: DuCard,
    about: DuAbout,
  },
  // システム・電源
  {
    id: "system_profiler.overview",
    cmd: "system_profiler",
    nameKey: "commands.system_profiler.label",
    control: SystemProfilerCard,
    about: SystemProfilerAbout,
  },
  {
    id: "pmset.apply",
    cmd: "pmset",
    nameKey: "commands.pmset.label",
    control: PmsetCard,
    about: PmsetAbout,
  },
  // ユーザー・ログイン
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
  // 開発ツール
  {
    id: "git.repo",
    cmd: "git",
    nameKey: "commands.git.label",
    control: GitCard,
    about: GitAbout,
  },
  {
    id: "docker.builderPrune",
    cmd: "docker",
    nameKey: "commands.docker.label",
    control: DockerCard,
    about: DockerAbout,
  },
];
