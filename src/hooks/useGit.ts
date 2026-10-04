import { useEffect, useMemo, useState } from "react";
import { api } from "../api";
import { tok, tokensToString, type CmdToken } from "../commands/tokens";
import type { GitSnapshot } from "../types";
import { useRunner } from "./useRunner";

export type GitSub = "status" | "log" | "branches" | "remotes";

export const GIT_SUBS: GitSub[] = ["status", "log", "branches", "remotes"];

const HISTORY_KEY = "manmen:git-dir-history";
const HISTORY_MAX = 8;

function loadHistory(): string[] {
  try {
    const raw = localStorage.getItem(HISTORY_KEY);
    const parsed: unknown = raw ? JSON.parse(raw) : [];
    return Array.isArray(parsed) ? parsed.filter((item): item is string => typeof item === "string") : [];
  } catch {
    return [];
  }
}

function saveHistory(dirs: string[]) {
  try {
    localStorage.setItem(HISTORY_KEY, JSON.stringify(dirs.slice(0, HISTORY_MAX)));
  } catch {
    // 保存できない環境では履歴なしで動く
  }
}

// Rust の git.rs が組み立てるコマンドと一致させること。
function buildTokens(sub: GitSub, dir: string): CmdToken[] {
  const target = dir.trim() ? dir.trim() : "…";
  const args =
    sub === "status"
      ? ["status", "--short", "--branch"]
      : sub === "log"
        ? ["log", "--oneline", "-n", "20"]
        : sub === "branches"
          ? ["branch", "-a"]
          : ["remote", "-v"];
  return [
    tok("git", "cmd"),
    tok("-C", "flag"),
    tok(target, dir.trim() ? "value" : "placeholder", "dir"),
    ...args.map((arg) => tok(arg, arg.startsWith("-") ? "flag" : "sub", "sub")),
  ];
}

export function useGit() {
  const [sub, setSub] = useState<GitSub>("status");
  const [dir, setDir] = useState("");
  const [detected, setDetected] = useState<string[]>([]);
  const [history, setHistory] = useState<string[]>(loadHistory);
  const runner = useRunner<GitSnapshot>();

  // よくある置き場のリポジトリを1回だけ列挙する
  useEffect(() => {
    api
      .listGitRepos()
      .then(setDetected)
      .catch(() => {
        // 列挙できない環境では履歴と自由入力だけで動く
      });
  }, []);

  // 履歴を先に、検出分を後に (重複なし)
  const suggestions = useMemo(() => {
    const seen = new Set<string>();
    const out: string[] = [];
    for (const candidate of [...history, ...detected]) {
      if (!seen.has(candidate)) {
        seen.add(candidate);
        out.push(candidate);
      }
    }
    return out;
  }, [history, detected]);

  const valid = dir.trim() !== "";
  const tokens = buildTokens(sub, dir);
  const preview = tokensToString(tokens);

  async function execute() {
    if (!valid) return;
    const target = dir.trim();
    const res = await runner.run(() => api.getGit(sub, target), preview);
    if (res?.success) {
      setHistory((prev) => {
        const next = [target, ...prev.filter((item) => item !== target)];
        saveHistory(next);
        return next.slice(0, HISTORY_MAX);
      });
    }
  }

  return { sub, setSub, dir, setDir, suggestions, runner, tokens, preview, valid, execute };
}
