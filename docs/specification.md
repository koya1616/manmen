# macOS CLI GUI Manager 開発仕様書

## 1. 概要

macOS標準のCLIコマンドをGUIから安全かつ直感的に操作できるmacOSデスクトップアプリケーションを開発する。

「macOSには強力なCLIが存在するが、CLIを知らないと使いにくい」という問題を解決する「CLI → GUI変換レイヤー」を目指す。

ユーザーはターミナルでコマンドを覚えて入力する必要なく、GUI上で以下を完結できる:

1. コマンドを検索
2. 用途・説明を確認
3. パラメータを入力
4. 実行されるCLIコマンドを確認
5. 権限認証
6. 実行
7. 結果（stdout / stderr / exit code）を確認

## 2. 技術スタック

### Frontend

- React / TypeScript / Vite
- Vite+ (Vite / Vitest / Oxlint / Oxfmt / Rolldown / Vite Task) - 統一エントリポイント
  - `vp dev` / `vp check` (format/lint/type check) / `vp test` / `vp build`
  - 設定は `vite.config.ts` に集約（`vitest.config.ts`は原則作成しない）
- react-i18next / i18next (ja/en対応、デフォルトja、選択はlocalStorage保存)

### Desktop Runtime

- Tauri 2 (Frontend ↔ Rust Core接続)

### Backend

- Rust / Cargo / serde / serde_json / rusqliteまたはSQLx / tokio(必要な場合のみ)

### macOS Native Layer

- Swift / Swift Package Manager / Apple純正 Framework (AuthorizationServices, Security, Keychain Services, UserNotifications, ServiceManagement, ApplicationServices)
- SwiftはmacOS固有APIのAdapterとしてのみ使用

### Database

- SQLite

### Testing / CI

- Frontend: Vitest / React Testing Library
- Rust: cargo test
- Swift: Swift Testing / XCTest
- CI: GitHub Actions

## 3. 初期実装スコープ

初期で実装するのは以下の1機能のみとする。

### 対象コマンド

```sh
sudo pmset -a disablesleep 1  # スリープ抑止 ON
sudo pmset -a disablesleep 0  # スリープ抑止 OFF
```

### 要件

- GUI上でスリープ抑止のON/OFFを切り替えられること
- 実行前に生成されるコマンドをプレビュー表示すること
- `requiresAdmin: true` として管理者権限を要求し、macOS標準の認証ダイアログを経由して実行すること
- 初回のみパスワードを入力し、アプリ起動中は2回目以降パスワードなしで実行できること（sudoタイムスタンプを利用、パスワードの保存はしない）
- 実行結果 (exit code / stdout / stderr) を表示すること

上記以外のコマンド・機能は本仕様書のスコープ外とする。
