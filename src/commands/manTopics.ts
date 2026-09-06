// manプルダウンの候補。すべて実機の `man -w` で存在確認済み。
// 追加は対応グループの topics に1行足すだけ。

export interface ManTopicGroup {
  labelKey: string;
  topics: string[];
}

export const MAN_TOPIC_GROUPS: ManTopicGroup[] = [
  {
    labelKey: "manGroups.power",
    topics: ["pmset", "caffeinate", "systemsetup", "shutdown"],
  },
  {
    labelKey: "manGroups.system",
    topics: [
      "system_profiler",
      "sysctl",
      "sw_vers",
      "uname",
      "uptime",
      "hostinfo",
      "launchctl",
      "log",
      "sysdiagnose",
      "softwareupdate",
      "nvram",
      "powermetrics",
    ],
  },
  {
    labelKey: "manGroups.network",
    topics: [
      "networksetup",
      "scutil",
      "ifconfig",
      "ping",
      "traceroute",
      "netstat",
      "route",
      "arp",
      "dig",
      "host",
      "nslookup",
      "networkQuality",
      "ssh",
      "scp",
      "ssh-keygen",
    ],
  },
  {
    labelKey: "manGroups.disk",
    topics: [
      "diskutil",
      "hdiutil",
      "df",
      "du",
      "mount",
      "umount",
      "tmutil",
      "fsck",
      "installer",
      "pkgutil",
    ],
  },
  {
    labelKey: "manGroups.files",
    topics: [
      "ls",
      "cp",
      "mv",
      "rm",
      "mkdir",
      "touch",
      "cat",
      "open",
      "find",
      "grep",
      "tar",
      "zip",
      "unzip",
      "chmod",
      "chown",
      "ln",
      "ditto",
      "xattr",
      "mdls",
      "mdfind",
      "defaults",
    ],
  },
  {
    labelKey: "manGroups.process",
    topics: ["ps", "top", "kill", "killall", "pkill"],
  },
  {
    labelKey: "manGroups.security",
    topics: [
      "sudo",
      "su",
      "id",
      "whoami",
      "dscl",
      "security",
      "codesign",
      "spctl",
      "fdesetup",
      "passwd",
      "who",
      "w",
      "last",
      "groups",
    ],
  },
  {
    labelKey: "manGroups.dev",
    topics: ["git", "make", "clang", "xcode-select", "xcrun", "python3"],
  },
  {
    labelKey: "manGroups.misc",
    topics: [
      "man",
      "apropos",
      "whatis",
      "say",
      "screencapture",
      "pbcopy",
      "pbpaste",
      "date",
      "env",
      "which",
    ],
  },
];

export const DEFAULT_MAN_TOPIC = "pmset";
