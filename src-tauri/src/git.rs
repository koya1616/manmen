//! `git` の参照系の定義・実行・出力パース。管理者権限は不要。
//! 対象は `status` / `log` / `branches` / `remotes` のみ。書き込み系
//! (`add` / `commit` / `push` 等) は受け付けない。対象ディレクトリは
//! 存在するディレクトリであることだけ検証し、`git -C` で指定する
//! (シェルを介さないため展開・注入は起きない)。

use serde::{Deserialize, Serialize};

use crate::privileged;

/// 許可するサブコマンド。Frontend の `GitSub` と一致させること。
pub const ALLOWED_SUBS: &[&str] = &["status", "log", "branches", "remotes"];

/// リポジトリ走査の起点 (存在するものだけ見る)。
const SCAN_ROOTS: &[&str] = &[
    "study",
    "src",
    "projects",
    "work",
    "dev",
    "repos",
    "repositories",
    "code",
    "Documents",
    "Desktop",
];

/// 走査の深さと読むディレクトリ数の上限 (暴走防止)。
const SCAN_MAX_DEPTH: usize = 3;
const SCAN_MAX_DIRS: usize = 3000;

/// `log` の取得件数 (固定)。
const LOG_COUNT: usize = 20;

/// Frontend の `getGit` 引数とフィールドを一致させること。
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitQuery {
    #[serde(default = "default_sub")]
    pub sub: String,
    #[serde(default)]
    pub dir: String,
}

fn default_sub() -> String {
    "status".to_string()
}

/// 変更ファイルの1行。Frontend の `GitFile` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GitFile {
    pub xy: String,
    pub path: String,
}

/// `get_git status` の返却値。Frontend の `GitStatusSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct GitStatusSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub dir: String,
    pub branch: String,
    pub tracking: String,
    pub files: Vec<GitFile>,
    pub count: usize,
    pub stderr: String,
}

/// コミットの1件。Frontend の `GitCommit` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GitCommit {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub subject: String,
}

/// `get_git log` の返却値。Frontend の `GitLogSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct GitLogSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub dir: String,
    pub commits: Vec<GitCommit>,
    pub count: usize,
    pub stderr: String,
}

/// ブランチの1件。Frontend の `GitBranch` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GitBranch {
    pub name: String,
    pub current: bool,
    pub remote: bool,
}

/// `get_git branches` の返却値。Frontend の `GitBranchesSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct GitBranchesSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub dir: String,
    pub branches: Vec<GitBranch>,
    pub count: usize,
    pub stderr: String,
}

/// リモートの1件。Frontend の `GitRemote` とフィールドを一致させること。
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct GitRemote {
    pub name: String,
    pub url: String,
    pub kind: String,
}

/// `get_git remotes` の返却値。Frontend の `GitRemotesSnapshot` と一致させること。
#[derive(Debug, Clone, Serialize)]
pub struct GitRemotesSnapshot {
    pub success: bool,
    pub exit_code: i32,
    pub command: String,
    pub dir: String,
    pub remotes: Vec<GitRemote>,
    pub count: usize,
    pub stderr: String,
}

/// Frontend へ返す共用体。`sub` ごとに中身が変わる。
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum GitResult {
    Status(GitStatusSnapshot),
    Log(GitLogSnapshot),
    Branches(GitBranchesSnapshot),
    Remotes(GitRemotesSnapshot),
}

pub fn get_snapshot(query: GitQuery) -> Result<GitResult, String> {
    #[cfg(not(target_os = "macos"))]
    {
        let _ = query;
        return Err("git is only supported on macOS".to_string());
    }

    #[cfg(target_os = "macos")]
    return run_snapshot(query);
}

#[cfg(target_os = "macos")]
fn run_snapshot(query: GitQuery) -> Result<GitResult, String> {
    let sub = query.sub.trim().to_string();
    if !ALLOWED_SUBS.contains(&sub.as_str()) {
        return Err(format!("サブコマンドが不正です: {sub}"));
    }
    let dir = validate_dir(&query.dir)?;
    match sub.as_str() {
        "status" => run_status(&dir).map(GitResult::Status),
        "log" => run_log(&dir).map(GitResult::Log),
        "branches" => run_branches(&dir).map(GitResult::Branches),
        _ => run_remotes(&dir).map(GitResult::Remotes),
    }
}

/// 存在するディレクトリのみ通す。空・存在しないパスは受け付けない。
fn validate_dir(raw: &str) -> Result<String, String> {
    let dir = raw.trim().to_string();
    if dir.is_empty() {
        return Err("ディレクトリを入力してください".to_string());
    }
    if !std::path::Path::new(&dir).is_dir() {
        return Err(format!("ディレクトリが見つかりません: {dir}"));
    }
    Ok(dir)
}

/// よくある置き場を浅く走査し、リポジトリ top を列挙する。
/// `.git` (実体・worktree の参照ファイルどちらも) と `*.git` (ベア) を拾う。
/// 隠しディレクトリ・シンボリックリンクは辿らない。
pub fn list_repos() -> Vec<String> {
    let home = std::env::var("HOME").unwrap_or_default();
    if home.is_empty() {
        return Vec::new();
    }
    let base = std::path::Path::new(&home);
    let mut out = Vec::new();
    let mut budget = SCAN_MAX_DIRS;
    // ホーム直下のリポジトリも拾う (深さ1まで)
    scan_root(base, 1, &mut out, &mut budget);
    for root in SCAN_ROOTS {
        let dir = base.join(root);
        if dir.is_dir() {
            scan_root(&dir, SCAN_MAX_DEPTH, &mut out, &mut budget);
        }
        if budget == 0 {
            break;
        }
    }
    out.sort();
    out.dedup();
    out
}

fn scan_root(root: &std::path::Path, max_depth: usize, out: &mut Vec<String>, budget: &mut usize) {
    if !root.is_dir() {
        return;
    }
    // ベアリポジトリ (`foo.git`) はそのまま候補
    if root
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".git"))
    {
        out.push(root.to_string_lossy().into_owned());
        return;
    }
    let mut stack = vec![(root.to_path_buf(), 0)];
    while let Some((dir, depth)) = stack.pop() {
        if *budget == 0 {
            return;
        }
        *budget -= 1;
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        let mut subdirs = Vec::new();
        let mut is_repo = false;
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            if name == ".git" {
                is_repo = true;
                break;
            }
            // ベアリポジトリ (`foo.git`) は潜らず候補にする
            if name.ends_with(".git") {
                out.push(path.to_string_lossy().into_owned());
                continue;
            }
            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if is_dir && !name.starts_with('.') {
                subdirs.push(path);
            }
        }
        if is_repo {
            out.push(dir.to_string_lossy().into_owned());
            continue;
        }
        if depth < max_depth {
            for sub in subdirs {
                stack.push((sub, depth + 1));
            }
        }
    }
}

fn run_git(dir: &str, args: &[&str], preview: String) -> Result<crate::types::CommandResult, String> {
    let mut full = vec!["-C", dir];
    full.extend(args);
    privileged::execute_plain("git", &full, preview)
}

/// `git -C <dir> status --porcelain=v1 --branch`
#[cfg(target_os = "macos")]
fn run_status(dir: &str) -> Result<GitStatusSnapshot, String> {
    let preview = format!("git -C {dir} status --short --branch");
    let output = run_git(dir, &["status", "--porcelain=v1", "--branch"], preview.clone())?;
    let (branch, tracking, files) = parse_status(&output.stdout);
    Ok(GitStatusSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        dir: dir.to_string(),
        branch,
        tracking,
        count: files.len(),
        files,
        stderr: output.stderr.trim().to_string(),
    })
}

/// 1行目の `## ...` をブランチ情報、残りを `XY path` として読む。
fn parse_status(stdout: &str) -> (String, String, Vec<GitFile>) {
    let mut lines = stdout.lines();
    let (mut branch, mut tracking) = (String::new(), String::new());
    if let Some(head) = lines.next() {
        let head = head.strip_prefix("## ").unwrap_or(head);
        // `main...origin/main [ahead 1]` / `main` / `No commits yet on main`
        let (left, track) = match head.split_once(" [") {
            Some((l, r)) => (l, r.trim_end_matches(']')),
            None => (head, ""),
        };
        tracking = track.to_string();
        branch = match left.split_once("...") {
            Some((b, _)) => b.to_string(),
            None => left.strip_prefix("No commits yet on ").unwrap_or(left).to_string(),
        };
    }
    let files = lines
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            // `XY path` (`R  old -> new` はそのまま載せる)
            let (xy, path) = line.split_at_checked(2)?;
            let path = path.trim().to_string();
            if path.is_empty() {
                return None;
            }
            Some(GitFile {
                xy: xy.to_string(),
                path,
            })
        })
        .collect();
    (branch, tracking, files)
}

/// `git -C <dir> log --pretty=... -n 20` (`\x1f` 区切りで崩れにくくする)
#[cfg(target_os = "macos")]
fn run_log(dir: &str) -> Result<GitLogSnapshot, String> {
    let preview = format!("git -C {dir} log --oneline -n {LOG_COUNT}");
    let format = "%h%x1f%an%x1f%ad%x1f%s";
    let count = LOG_COUNT.to_string();
    let output = run_git(
        dir,
        &[
            "log",
            &format!("--pretty=format:{format}"),
            "--date=short",
            "--no-color",
            "-n",
            &count,
        ],
        preview.clone(),
    )?;
    let commits = parse_log(&output.stdout);
    Ok(GitLogSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        dir: dir.to_string(),
        count: commits.len(),
        commits,
        stderr: output.stderr.trim().to_string(),
    })
}

fn parse_log(stdout: &str) -> Vec<GitCommit> {
    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.splitn(4, '\x1f');
            Some(GitCommit {
                hash: parts.next()?.to_string(),
                author: parts.next()?.to_string(),
                date: parts.next()?.to_string(),
                subject: parts.next()?.to_string(),
            })
        })
        .collect()
}

/// `git -C <dir> branch -a --no-color`
#[cfg(target_os = "macos")]
fn run_branches(dir: &str) -> Result<GitBranchesSnapshot, String> {
    let preview = format!("git -C {dir} branch -a");
    let output = run_git(dir, &["branch", "-a", "--no-color"], preview.clone())?;
    let branches = parse_branches(&output.stdout);
    Ok(GitBranchesSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        dir: dir.to_string(),
        count: branches.len(),
        branches,
        stderr: output.stderr.trim().to_string(),
    })
}

/// `* main` を現行、`remotes/...` をリモート、`->` の別名行は捨てる。
fn parse_branches(stdout: &str) -> Vec<GitBranch> {
    stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.contains(" -> "))
        .map(|line| {
            let (current, name) = match line.strip_prefix("* ") {
                Some(rest) => (true, rest),
                None => (false, line),
            };
            let (remote, name) = match name.strip_prefix("remotes/") {
                Some(rest) => (true, rest),
                None => (false, name),
            };
            GitBranch {
                name: name.to_string(),
                current,
                remote,
            }
        })
        .collect()
}

/// `git -C <dir> remote -v`
#[cfg(target_os = "macos")]
fn run_remotes(dir: &str) -> Result<GitRemotesSnapshot, String> {
    let preview = format!("git -C {dir} remote -v");
    let output = run_git(dir, &["remote", "-v"], preview.clone())?;
    let remotes = parse_remotes(&output.stdout);
    Ok(GitRemotesSnapshot {
        success: output.success,
        exit_code: output.exit_code,
        command: preview,
        dir: dir.to_string(),
        count: remotes.len(),
        remotes,
        stderr: output.stderr.trim().to_string(),
    })
}

/// `origin\tgit@... (fetch)` を3列で読む。
fn parse_remotes(stdout: &str) -> Vec<GitRemote> {
    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            let name = parts.next()?.to_string();
            let url = parts.next()?.to_string();
            let kind = parts
                .next()
                .unwrap_or("(fetch)")
                .trim_matches(|c| c == '(' || c == ')')
                .to_string();
            Some(GitRemote { name, url, kind })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{parse_branches, parse_log, parse_remotes, parse_status, validate_dir};

    #[test]
    fn parses_status() {
        let (branch, tracking, files) = parse_status(
            "## main...origin/main [ahead 1]\nM  src/a.ts\n?? new.txt\nR  old -> new\n",
        );
        assert_eq!(branch, "main");
        assert_eq!(tracking, "ahead 1");
        assert_eq!(files.len(), 3);
        assert_eq!(files[0].xy, "M ");
        assert_eq!(files[0].path, "src/a.ts");
        assert_eq!(files[1].xy, "??");
        assert_eq!(files[2].path, "old -> new");
    }

    #[test]
    fn parses_bare_branch() {
        let (branch, tracking, _) = parse_status("## main\n");
        assert_eq!(branch, "main");
        assert_eq!(tracking, "");
    }

    #[test]
    fn parses_log() {
        let commits = parse_log("abc1234\x1fTaro\x1f2026-10-01\x1fAdd feature\n");
        assert_eq!(commits.len(), 1);
        assert_eq!(commits[0].hash, "abc1234");
        assert_eq!(commits[0].subject, "Add feature");
    }

    #[test]
    fn parses_branches() {
        let branches = parse_branches("* main\n  feature\n  remotes/origin/HEAD -> origin/main\n  remotes/origin/main\n");
        assert_eq!(branches.len(), 3);
        assert!(branches[0].current);
        assert!(!branches[0].remote);
        assert!(branches[2].remote);
        assert_eq!(branches[2].name, "origin/main");
    }

    #[test]
    fn parses_remotes() {
        let remotes = parse_remotes("origin\tgit@github.com:a/b.git (fetch)\norigin\tgit@github.com:a/b.git (push)\n");
        assert_eq!(remotes.len(), 2);
        assert_eq!(remotes[0].name, "origin");
        assert_eq!(remotes[0].kind, "fetch");
        assert_eq!(remotes[1].kind, "push");
    }

    #[test]
    fn rejects_bad_dirs() {
        assert!(validate_dir("").is_err());
        assert!(validate_dir("/definitely/not/here-xyz").is_err());
    }

    /// 走査用の仮ディレクトリ (`repo/.git`・worktree 参照・ベア・隠し・深い nested)。
    fn scan_fixture() -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("manmen-git-scan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for dir in [
            "repo-a/.git",
            "org/repo-b/.git",
            "wt",
            "bare.git",
            ".hidden/repo-c/.git",
            "deep/1/2/3/repo-d/.git",
        ] {
            std::fs::create_dir_all(root.join(dir)).unwrap();
        }
        // worktree の `.git` はファイル
        std::fs::write(root.join("wt/.git"), "gitdir: /elsewhere\n").unwrap();
        root
    }

    #[test]
    fn scans_repos() {
        let root = scan_fixture();
        let mut out = Vec::new();
        let mut budget = 1000;
        super::scan_root(&root, 3, &mut out, &mut budget);
        out.sort();
        let names: Vec<String> = out
            .iter()
            .map(|p| {
                std::path::Path::new(p)
                    .strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        // 通常・入れ子・worktree 参照・ベアを拾い、隠しと深すぎは落とす
        assert!(names.contains(&"repo-a".to_string()), "{names:?}");
        assert!(names.contains(&"org/repo-b".to_string()), "{names:?}");
        assert!(names.contains(&"wt".to_string()), "{names:?}");
        assert!(names.contains(&"bare.git".to_string()), "{names:?}");
        assert!(!names.iter().any(|n| n.contains(".hidden")), "{names:?}");
        assert!(!names.iter().any(|n| n.contains("repo-d")), "{names:?}");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// 自分自身のリポジトリで実走する。`.git` がなければ飛ばす。
    fn own_repo() -> Option<String> {
        let mut dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        loop {
            if dir.join(".git").exists() {
                return dir.to_string_lossy().into_owned().into();
            }
            if !dir.pop() {
                return None;
            }
        }
    }

    #[test]
    fn runs_status_in_own_repo() {
        let Some(dir) = own_repo() else { return };
        let result = super::run_status(&dir).expect("git status");
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(!result.branch.is_empty());
    }

    #[test]
    fn runs_log_in_own_repo() {
        let Some(dir) = own_repo() else { return };
        let result = super::run_log(&dir).expect("git log");
        assert!(result.success, "stderr: {}", result.stderr);
        assert!(!result.commits.is_empty());
    }

    #[test]
    fn runs_branches_and_remotes_in_own_repo() {
        let Some(dir) = own_repo() else { return };
        let branches = super::run_branches(&dir).expect("git branch");
        assert!(branches.success, "stderr: {}", branches.stderr);
        let remotes = super::run_remotes(&dir).expect("git remote");
        assert!(remotes.success, "stderr: {}", remotes.stderr);
    }
}
