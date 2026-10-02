import { invoke } from "@tauri-apps/api/core";
import type {
  CommandResult,
  DuSnapshot,
  InspectSnapshot,
  LsSnapshot,
  PmsetState,
  ManpageDocument,
  TopSnapshot,
  VersionSnapshot,
  ContainerSnapshot,
  ImageSnapshot,
  NetworkSnapshot,
  VolumeSnapshot,
  SystemDfSnapshot,
  SystemInfoSnapshot,
} from "./types";

// IPC コマンド名はここに集約する。Rust の commands.rs と一致させること。
export const api = {
  getPmset: () => invoke<PmsetState>("get_pmset"),
  setPmset: (scope: string, setting: string, value: string) =>
    invoke<CommandResult>("set_pmset", { scope, setting, value }),
  getManpage: (topic: string) =>
    invoke<ManpageDocument>("get_manpage", { topic }),
  getTop: (query: {
    sortKey: string;
    sortOrder: string;
    secondaryKey: string;
    count: number;
    countMode: string;
    noFrameworks: boolean;
    memoryMap: boolean;
    swap: boolean;
    user: string;
    pids: string;
    stats: string[];
    ncols: number | null;
  }) => invoke<TopSnapshot>("get_top", { query }),
  pruneDockerBuilder: (force: boolean, all: boolean) =>
    invoke<CommandResult>("prune_docker_builder", { force, all }),
  dockerBuilderDu: () => invoke<DuSnapshot>("docker_builder_du"),
  dockerBuilderLs: () => invoke<LsSnapshot>("docker_builder_ls"),
  dockerBuilderInspect: (name: string) =>
    invoke<InspectSnapshot>("docker_builder_inspect", { name }),
  dockerBuilderVersion: () =>
    invoke<VersionSnapshot>("docker_builder_version"),
  dockerContainers: () => invoke<ContainerSnapshot>("docker_containers"),
  dockerImages: () => invoke<ImageSnapshot>("docker_images"),
  dockerNetworks: () => invoke<NetworkSnapshot>("docker_networks"),
  dockerVolumes: () => invoke<VolumeSnapshot>("docker_volumes"),
  dockerSystemDf: () => invoke<SystemDfSnapshot>("docker_system_df"),
  dockerSystemInfo: () => invoke<SystemInfoSnapshot>("docker_system_info"),
  setRemember: (enabled: boolean) =>
    invoke<void>("set_remember", { enabled }),
};
