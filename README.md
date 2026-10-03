# Manmen

macOS の CLI を GUI から操作するデスクトップアプリ（Tauri + React + TypeScript）。

## 前提

- macOS
- Node.js
- pnpm (`packageManager: pnpm@12.8.1`)
- Rust / Cargo
- Xcode Command Line Tools (`xcode-select --install`)

## セットアップ

```sh
git clone https://github.com/koya1616/manmen.git
cd manmen

# pnpm を有効化（未導入の場合）
corepack enable

pnpm install
```

### Web プレビューで動かす

```sh
pnpm dev
```

### アプリとして動かす（Tauri）

```sh
pnpm tauri:dev
```

### ビルド

```sh
pnpm build
pnpm tauri:build
```

ビルド成果物: `src-tauri/target/release/bundle/macos/Manmen.app`

### ログイン時同期の有効化（任意）

`origin/main` を `/Applications/Manmen.app` に自動反映する:

```sh
zsh scripts/install-sync.sh
```
