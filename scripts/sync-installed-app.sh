#!/bin/zsh
# origin/main と /Applications/Manmen.app を揃える。
# 作業ツリーが main で綺麗なときだけチェックアウトを fast-forward する。
# それ以外のときは worktree でビルドし、手元の変更は残す。
set -euo pipefail

export PATH="$HOME/.local/bin:$HOME/.nix-profile/bin:/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin"
export GIT_TERMINAL_PROMPT=0

REPO="$(cd "$(dirname "$0")/.." && pwd)"
SUPPORT="$HOME/Library/Application Support/manmen"
STAMP="$SUPPORT/installed-sha"
APP="/Applications/Manmen.app"
BUNDLE="$REPO/src-tauri/target/release/bundle/macos/Manmen.app"
LOCKDIR="$SUPPORT/sync.lockdir"
BUILD_DIR="$HOME/Library/Caches/manmen/src"

mkdir -p "$SUPPORT"

log() {
  printf '[%s] %s\n' "$(date '+%Y-%m-%d %H:%M:%S')" "$*"
}

notify() {
  osascript -e "display notification \"$1\" with title \"Manmen\"" >/dev/null 2>&1 || true
}

if ! mkdir "$LOCKDIR" 2>/dev/null; then
  oldpid="$(cat "$LOCKDIR/pid" 2>/dev/null || true)"
  if [[ -n "$oldpid" ]] && kill -0 "$oldpid" 2>/dev/null; then
    log "別の同期が実行中のため終了します"
    exit 0
  fi
  rm -rf "$LOCKDIR"
  mkdir "$LOCKDIR"
fi
echo $$ > "$LOCKDIR/pid"
trap 'rm -rf "$LOCKDIR"' EXIT

cd "$REPO"
git fetch origin main
remote_sha="$(git rev-parse origin/main)"
installed_sha="$(cat "$STAMP" 2>/dev/null || true)"
branch="$(git branch --show-current)"
dirty="$(git status --porcelain)"

if [[ "$branch" == "main" && -z "$dirty" ]]; then
  if ! git pull --ff-only origin main; then
    log "fast-forward できないため、チェックアウトはそのままにします"
  fi
  remote_sha="$(git rev-parse origin/main)"
fi

if [[ "$remote_sha" == "$installed_sha" ]]; then
  log "アプリは origin/main ($remote_sha) と一致しています"
  exit 0
fi

log "origin/main ($remote_sha) をアプリに反映します"
notify "main の更新をアプリに反映しています"

build_dir="$REPO"
if [[ "$branch" != "main" || -n "$dirty" || "$(git rev-parse HEAD)" != "$remote_sha" ]]; then
  log "作業ツリーを残し、別ディレクトリでビルドします"
  if [[ ! -d "$BUILD_DIR/.git" ]]; then
    mkdir -p "$(dirname "$BUILD_DIR")"
    git worktree add --detach "$BUILD_DIR" "$remote_sha"
  else
    git -C "$BUILD_DIR" checkout --detach "$remote_sha"
    git -C "$BUILD_DIR" reset --hard "$remote_sha"
  fi
  build_dir="$BUILD_DIR"
  BUNDLE="$BUILD_DIR/src-tauri/target/release/bundle/macos/Manmen.app"
fi

(
  cd "$build_dir"
  pnpm install --frozen-lockfile
  pnpm tauri:build
)

if [[ ! -d "$BUNDLE" ]]; then
  log "ビルド成果物が見つかりません: $BUNDLE"
  notify "アプリの更新に失敗しました"
  exit 1
fi

was_running=0
if pgrep -f '/Applications/Manmen.app/Contents/MacOS/manmen' >/dev/null 2>&1; then
  was_running=1
  osascript -e 'tell application "Manmen" to quit' >/dev/null 2>&1 || true
  for _ in {1..20}; do
    pgrep -f '/Applications/Manmen.app/Contents/MacOS/manmen' >/dev/null 2>&1 || break
    sleep 0.5
  done
  if pgrep -f '/Applications/Manmen.app/Contents/MacOS/manmen' >/dev/null 2>&1; then
    log "アプリが終了しないため置き換えを中止しました"
    notify "アプリの更新を中止しました"
    exit 1
  fi
fi

rm -rf "$APP"
ditto "$BUNDLE" "$APP"
echo "$remote_sha" > "$STAMP"
log "インストール完了: $remote_sha"

if [[ "$was_running" -eq 1 ]]; then
  open "$APP"
fi
notify "アプリを最新の main に更新しました"
