#!/bin/zsh
# ログイン時に sync-installed-app.sh を1回動かす。
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
SCRIPT="$REPO/scripts/sync-installed-app.sh"
PLIST="$HOME/Library/LaunchAgents/com.manmen.sync.plist"
DOMAIN="gui/$(id -u)"
SUPPORT="$HOME/Library/Application Support/manmen"

chmod +x "$SCRIPT"
mkdir -p "$HOME/Library/LaunchAgents" "$SUPPORT" "$HOME/Library/Logs"

cat > "$PLIST" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>Label</key>
  <string>com.manmen.sync</string>
  <key>ProgramArguments</key>
  <array>
    <string>/bin/zsh</string>
    <string>$SCRIPT</string>
  </array>
  <key>RunAtLoad</key>
  <true/>
  <key>StandardOutPath</key>
  <string>$HOME/Library/Logs/manmen-sync.log</string>
  <key>StandardErrorPath</key>
  <string>$HOME/Library/Logs/manmen-sync.log</string>
</dict>
</plist>
EOF

# いま入っているアプリは直近ビルドと一致するので、起動直後に再ビルドしない。
if [[ ! -f "$SUPPORT/installed-sha" && -d /Applications/Manmen.app ]]; then
  git -C "$REPO" fetch origin main
  git -C "$REPO" rev-parse origin/main > "$SUPPORT/installed-sha"
fi

launchctl bootout "$DOMAIN" "$PLIST" >/dev/null 2>&1 || true
launchctl bootstrap "$DOMAIN" "$PLIST"
launchctl enable "$DOMAIN/com.manmen.sync"
echo "有効にしました: $PLIST（ログイン時のみ）"
