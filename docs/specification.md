macOS CLI GUI Manager 開発仕様書

1. プロダクト概要

macOSに標準搭載されているCLIコマンドやmacOS固有のシステム機能を、GUIから安全かつ直感的に操作できるmacOSデスクトップアプリケーションを開発する。

ユーザーがターミナルでコマンド名やオプションを覚えて入力する必要をなくし、

1. コマンドを検索する
2. コマンドの用途・説明を確認する
3. GUIフォームからパラメータを入力する
4. 実際に実行されるCLIコマンドを確認する
5. 必要に応じてmacOSの権限認証を行う
6. コマンドを実行する
7. stdout / stderr / exit codeを確認する
8. 実行履歴を確認する

という一連の操作をGUIだけで完結させる。

本プロダクトはターミナルの代替ではない。

「macOSには強力なCLIが存在するが、CLIを知らないと使いにくい」という問題を解決する「CLI → GUI変換レイヤー」を目指す。

⸻

2. プロダクトコンセプト

2.1 基本コンセプト

「macOSのCLIを知らなくても、Macの高度な機能を操作できる」

ユーザー体験:

Search
→ Discover
→ Understand
→ Configure
→ Preview
→ Authorize
→ Execute
→ Result

⸻

2.2 将来的なコンセプト

最終的には、macOSのmanページやCLI仕様からCommand Definitionを生成し、それをGUIフォームへ自動変換できる仕組みを構築する。

最終イメージ:

man command
↓
Parser
↓
Command Definition
↓
GUI Generator
↓
GUI

これにより、新しいコマンドを追加するたびにVue/Reactコンポーネントを個別実装する必要をなくす。

⸻

3. 技術スタック

Frontend

* React
* TypeScript
* Vite
* Vite+
    * Vite
    * Vitest
    * Oxlint
    * Oxfmt
    * Rolldown
    * Vite Task
* react-i18next（多言語対応）
* i18next
* i18next-browser-languagedetector

Vite+をFrontend Toolchainの統一エントリポイントとして使用する。

基本的な開発コマンド:

vp dev
vp check
vp test
vp build

Vite+のvp checkでformat / lint / type checkを統合し、vp testでVitestを実行する。

Vite+の設定は原則としてルートのvite.config.tsへ集約する。

vitest.config.tsは原則作成しない。

多言語対応:

日本語（ja）と英語（en）をサポートする。
デフォルトは日本語。
ユーザーの選択はlocalStorageに保存する。
Command Definition JSONの翻訳はi18n側で管理する。

⸻

Desktop Runtime

* Tauri 2

FrontendとRust Coreを接続する。

⸻

Backend

* Rust
* Cargo
* serde
* serde_json
* rusqliteまたはSQLx
* tokioは必要な場合のみ利用

RustをアプリケーションのCore Layerとする。

⸻

macOS Native Layer

* Swift
* Swift Package Manager
* Apple純正Framework

主に以下を利用する。

* AuthorizationServices
* Security
* Keychain Services
* UserNotifications
* ServiceManagement
* ApplicationServices
* 必要に応じてその他のmacOS Framework

SwiftはmacOS固有APIのAdapterとして扱う。

⸻

Database

SQLite

保存対象:

* execution history
* favorites
* user preferences
* command metadata cache
* search index

⸻

Testing

Frontend:

* Vitest
* React Testing Library
* 必要に応じてPlaywright

Rust:

* cargo test

Swift:

* Swift Testing / XCTest

E2E:

* Playwright
* Tauriアプリの主要フローをテスト

⸻

CI/CD

GitHub Actions

実行:

* Frontend install
* vp check
* vp test
* cargo fmt --check
* cargo clippy
* cargo test
* Swift build/test
* Tauri build

将来的に:

* Apple Developer署名
* notarization
* DMG生成
* GitHub Release
* 自動アップデート

を追加する。

⸻

4. 全体アーキテクチャ

以下の4層に分離する。

Frontend Layer
↓
Tauri IPC
↓
Rust Core
↓
macOS Native Layer / macOS CLI

構成:

React
↓
Tauri invoke
↓
Rust
├── Command Registry
├── Command Builder
├── Validator
├── CLI Executor
├── History Repository
├── Search Engine
├── Security
└── Native Bridge
↓
Swift
↓
Apple Framework

⸻

5. 責務分離

React

担当:

* UI
* state
* form
* navigation
* search UI
* command detail
* result display
* history display
* settings

Reactから直接CLIを実行してはいけない。

⸻

Tauri

担当:

* Frontend ↔ Rust IPC
* Window management
* Application lifecycle
* Tauri permissions / capabilities
* Desktop integration

⸻

Rust

アプリケーションの中心。

担当:

* Command Definition
* Command Registry
* Command Search
* Argument Validation
* Command Builder
* Process Execution
* stdout / stderr
* exit code
* timeout
* cancellation
* history
* SQLite
* security
* Swift Bridge
* man parser

⸻

Swift

macOS固有機能のみ担当。

担当:

* AuthorizationServices
* Keychain
* Notification
* Accessibility
* Login Item
* privileged operation
* macOS固有API

Rustで完結できる処理をSwiftへ移さない。

⸻

6. Frontend構成

React + TypeScript。

実装構成:

src/
├── App.tsx
├── main.tsx
├── index.css
│
├── components/
│   ├── command/
│   │   └── CommandDetailView.tsx
│   ├── form/
│   │   └── DynamicForm.tsx
│   ├── result/
│   │   └── ExecutionResult.tsx
│   └── layout/
│       └── Sidebar.tsx
│
├── hooks/
│   ├── useCommands.ts
│   ├── useCommandExecution.ts
│   └── useSearch.ts
│
├── services/
│   └── tauri/
│       └── index.ts
│
├── types/
│   └── index.ts
│
└── i18n/
    ├── index.ts
    └── locales/
        ├── ja.json
        └── en.json

UIコンポーネントは機能単位で管理する。
i18nは日本語・英語をサポートし、Command Definitionの翻訳はi18n側で管理する。

⸻

7. Frontend State

グローバルstateは必要最小限にする。

管理対象:

* selected category
* selected command
* search query
* command form state
* execution state
* execution result
* sidebar state
* favorites
* application settings

サーバーStateとUI Stateを混同しない。

Tauri IPCをBackend APIとして扱う。

⸻

8. Tauri IPC

FrontendからRustへはTauri Commandを使用する。

例:

get_commands()

get_command(id)

search_commands(query)

get_categories()

validate_command(command_id, arguments)

build_command(command_id, arguments)

execute_command(command_id, arguments)

cancel_execution(execution_id)

get_execution_history()

get_execution(id)

favorite_command(command_id)

unfavorite_command(command_id)

get_settings()

update_settings(settings)

FrontendはCLIを直接実行しない。

⸻

9. Command Definition

本アプリの最重要データモデル。

Command Definitionを元にGUIを動的生成する。

例:

{
“id”: “macos.pmset”,
“command”: “pmset”,
“name”: “Power Management”,
“category”: “power”,
“description”: “Manage macOS power management settings.”,
“platform”: “macos”,
“requiresAdmin”: true,
“dangerous”: false,
“arguments”: []
}

⸻

10. Argument Definition

Argument Definition:

{
“id”: “sleep”,
“name”: “Sleep”,
“description”: “System sleep timeout.”,
“type”: “integer”,
“required”: false,
“default”: null,
“min”: 0,
“max”: 100000,
“unit”: “minutes”
}

対応Type:

* string
* integer
* number
* boolean
* enum
* path
* file
* directory
* duration
* multiple

⸻

11. GUI Dynamic Form

Argument DefinitionからReact UIを動的生成する。

integer
→ NumberInput

number
→ NumberInput

string
→ TextInput

boolean
→ Switch

enum
→ Select

path
→ PathPicker

file
→ FilePicker

directory
→ DirectoryPicker

duration
→ DurationInput

multiple
→ RepeatingInput

Command Definition追加時に基本的にはReactコードを変更しない。

⸻

12. Command Definitionの例

pmset:

{
“id”: “macos.pmset”,
“command”: “pmset”,
“name”: “Power Management”,
“category”: “power”,
“description”: “Configure power management settings.”,
“requiresAdmin”: true,
“arguments”: [
{
“id”: “sleep”,
“name”: “System Sleep”,
“type”: “integer”,
“unit”: “minutes”,
“min”: 0
},
{
“id”: “displaysleep”,
“name”: “Display Sleep”,
“type”: “integer”,
“unit”: “minutes”,
“min”: 0
},
{
“id”: “powernap”,
“name”: “Power Nap”,
“type”: “boolean”
}
]
}

GUI:

Power Management

System Sleep
[ 30 ] minutes

Display Sleep
[ 10 ] minutes

Power Nap
[ ON ]

Generated Command:

pmset -a sleep 30 displaysleep 10 powernap 1

[ Execute ]

⸻

13. Command Builder

Command BuilderはRustで実装する。

責務:

* 入力値を検証
* CLI argumentへ変換
* argument orderを決定
* commandを生成
* shell injectionを防止

Commandは文字列ではなくargv配列として扱う。

例:

program:

pmset

args:

[
“-a”,
“sleep”,
“30”,
“displaysleep”,
“10”
]

Rust Process:

Command::new(“pmset”)
.args(args)

shell経由で実行しない。

禁止:

sh -c “pmset …”

bash -c “…”

eval “…”

/bin/sh -c

⸻

14. Command Validation

実行前に以下を検証する。

* required
* type
* min
* max
* enum
* path
* file existence
* directory existence
* OS version
* permission
* dangerous flag
* argument compatibility

Validation Error:

{
“field”: “sleep”,
“code”: “INVALID_RANGE”,
“message”: “Sleep must be greater than or equal to 0.”
}

⸻

15. Command Execution

RustにCommandExecutorを実装する。

入力:

command_id
arguments

処理:

Command Definition取得
↓
Validation
↓
Command Build
↓
Permission Check
↓
Process Spawn
↓
stdout/stderr
↓
exit code
↓
Result
↓
History Save

⸻

16. Command Result

CommandResult:

{
“executionId”: “uuid”,
“success”: true,
“exitCode”: 0,
“stdout”: “…”,
“stderr”: “”,
“durationMs”: 120,
“startedAt”: “…”,
“finishedAt”: “…”
}

⸻

17. stdout / stderr

stdoutとstderrを別々に取得する。

GUIでは、

Output
Error

の2つを分けて表示できるようにする。

大量出力に備えてUIでは仮想スクロール等を利用する。

⸻

18. Process Cancellation

長時間実行されるCLIをキャンセル可能にする。

UI:

Running…

[ Cancel ]

Rust:

Child Processを保持する。

cancel_execution(execution_id)

を受けたら対象Processを終了する。

⸻

19. Timeout

Command Definitionごとにtimeoutを指定可能にする。

例:

{
“timeoutMs”: 30000
}

デフォルト:

30秒

長時間処理が必要なコマンドはCommand Definitionで上書き可能。

⸻

20. Permission Model

Command Definition:

requiresAdmin

を持つ。

false:

通常権限で実行。

true:

管理者権限が必要かチェック。

必要な場合:

Rust
↓
Swift Native Bridge
↓
AuthorizationServices
↓
macOS Authentication
↓
Privileged Operation

管理者パスワードをRust / Reactへ渡さない。

パスワードをDBへ保存しない。

⸻

21. Swift Native Bridge

Swift LayerにはNative APIを集約する。

例:

authorizePrivilegedOperation()

showNotification()

getKeychainItem()

setLoginItem()

checkAccessibilityPermission()

Swift APIはRustから呼び出せる薄いAdapterとして設計する。

⸻

22. Keychain

将来必要になる認証情報はmacOS Keychainを利用する。

禁止:

SQLiteへの平文保存

JSONへの保存

UserDefaultsへの秘密情報保存

環境変数への永続保存

⸻

23. Tauri Security

Tauriのcapability / permissionモデルを利用する。

Frontendへ不要な権限を与えない。

Frontendから任意のfilesystem / shell / process操作を許可しない。

CLI実行はRust側のホワイトリストCommand Definition経由に限定する。

⸻

24. Command Whitelist

任意の実行ファイルをユーザー入力で指定する機能をMVPでは提供しない。

Command Definitionに登録されたコマンドのみ実行する。

例:

{
“command”: “pmset”
}

実行可能なbinaryを固定する。

必要に応じてbinary pathをmacOS標準パスへ限定する。

⸻

25. Dangerous Command

Command Definition:

{
“dangerous”: true
}

の場合、通常のExecuteより強い確認UIを表示する。

例:

This command may modify system settings.

Generated command:

…

[Cancel]

[Execute]

さらに破壊的操作の場合は、

「I understand」

などの明示確認を要求する。

⸻

26. Command Search

検索対象:

* command
* name
* description
* category
* argument
* tags
* aliases
* documentation

例:

検索:

sleep

結果:

pmset
caffeinate
systemsetup

⸻

27. Search Ranking

検索順位:

1. command完全一致
2. command prefix
3. name完全一致
4. name prefix
5. description
6. argument
7. documentation

将来的に自然言語検索を追加する。

⸻

28. Categories

初期カテゴリ:

Power
Network
Disk
System
Display
Process
Security
User
Files
Development

Command Definitionからカテゴリを登録できる。

⸻

29. 初期Command

MVP:

Power

pmset
caffeinate

Network

networksetup
scutil

Disk

diskutil

System

defaults
system_profiler
launchctl

合計8コマンド程度から開始する。

⸻

30. Command Definitionの保存場所

Repository内:

commands/

commands/
├── power/
│   ├── pmset.json
│   └── caffeinate.json
├── network/
│   ├── networksetup.json
│   └── scutil.json
├── disk/
│   └── diskutil.json
└── system/
├── defaults.json
├── system_profiler.json
└── launchctl.json

Command DefinitionはGitで管理する。

ユーザー固有の設定はSQLiteへ保存する。

⸻

31. Versioning

Command Definitionにversionを持たせる。

{
“schemaVersion”: 1,
“id”: “macos.pmset”
}

将来Schemaが変更された場合にMigrationできるようにする。

⸻

32. macOS Version Compatibility

Command Definition:

{
“supportedOS”: {
“min”: “14.0”,
“max”: null
}
}

現在のmacOS versionを取得し、非対応Commandを表示しない、または警告表示する。

⸻

33. Documentation

Command Detailで以下を表示する。

* description
* syntax
* options
* examples
* warnings
* supported OS
* permissions

将来的にman pageを直接表示できるようにする。

⸻

34. man Parser

Phase 2以降で実装。

Rustから:

man pmset

などを取得する。

Parser:

man output
↓
section parser
↓
option parser
↓
argument parser
↓
Command Definition

完全自動生成ではなく、

Generated Definition
↓
Review / Edit
↓
Approved Definition

というフローを採用する。

⸻

35. Man Parserの制約

manページは自然言語に近い形式で記述されており、すべての引数を完全自動解析できるとは限らない。

そのためParserは、

* 高信頼情報
* 推測情報
* 未解析情報

を区別する。

自動生成されたCommand Definitionには、

generated: true

confidence:

high / medium / low

などのmetadataを持たせることを検討する。

⸻

36. Command Detail UI

レイアウト:

Sidebar
↓
Command List
↓
Command Detail

Command Detail:

名称
Description

Permission
Administrator Required

Arguments

[Sleep]
[30] minutes

[Display Sleep]
[10] minutes

[Power Nap]
[ON]

Generated Command

pmset -a sleep 30 displaysleep 10 powernap 1

[Copy]

[Execute]

⸻

37. Execute Confirmation

Executeを押すと確認画面を表示する。

例:

Execute Command?

pmset -a sleep 30 displaysleep 10 powernap 1

Permission:
Administrator

Warning:
This command modifies system power settings.

[Cancel]

[Continue]

⸻

38. Execution Result UI

Success:

✓ Command completed

Exit Code
0

Output
…

Error
None

Failure:

✕ Command failed

Exit Code
1

Output
…

Error
…

⸻

39. Execution History

SQLiteへ保存。

Table:

executions

columns:

id
command_id
command
arguments_json
generated_command
exit_code
success
stdout
stderr
duration_ms
started_at
finished_at

⸻

40. History UI

一覧:

Today

12:30
pmset -a sleep 30

12:10
networksetup -getinfo Wi-Fi

Yesterday

…

クリックすると詳細を表示する。

⸻

41. History Security

stdout / stderrに秘密情報が含まれる可能性があるため、将来的にCommand Definitionで、

sensitiveOutput: true

を指定できるようにする。

sensitiveOutput=trueの場合、履歴へのstdout/stderr保存を禁止またはマスキングする。

⸻

42. Favorites

CommandにFavoriteを設定できる。

ホーム画面:

Favorites

Power Management
Network Settings
Disk Utility
Launch Services

⸻

43. Home画面

起動時:

Search

What do you want to do?

[ Search commands ]

Favorites

Recent Commands

Categories

⸻

44. UI Design

macOSネイティブアプリらしいシンプルなUIを目指す。

基本:

* Sidebar
* Toolbar
* Search
* Cardsは必要最小限
* 大量の装飾を避ける
* キーボード操作を重視
* Dark Mode / Light Mode対応
* macOS system appearanceに追従

⸻

45. Keyboard Shortcuts

最低限:

Cmd + K
→ Command Search

Cmd + Enter
→ Execute

Cmd + C
→ Generated Command Copy

Cmd + ,
→ Settings

Esc
→ Modal close / cancel

⸻

46. Command Palette

将来的にCommand Paletteを実装する。

Cmd + K

例:

sleep

↓

Power Management
Caffeinate
System Setup

選択するとCommand Detailへ移動。

⸻

47. Menu Bar

将来的にMenu Bar Appへ対応する。

Menu Bar:

MacKit

Favorites
Recent Commands
Search Commands
Open App

ただしMVPでは通常のmacOSアプリのみ。

⸻

48. Notification

長時間処理が完了した場合、Swift Native Layer経由でmacOS Notificationを表示できるようにする。

例:

Command Completed

diskutil operation completed successfully.

⸻

49. SQLite設計

commandsは基本的にJSONをSource of Truthとする。

SQLiteは以下に限定:

* executions
* favorites
* settings
* search cache
* user overrides

Command Definition自体をSQLiteだけで管理しない。

⸻

50. User Override

将来的にユーザーがCommand Definitionの一部をカスタマイズできるようにする。

例:

default value

label

favorite

hidden

しかし元のCommand Definitionは変更しない。

User Overrideとして別管理する。

⸻

51. Plugin Architecture

将来的にCommunity Command Definitionを追加できるようにする。

Plugin:

plugin.json

commands/

*.json

将来的にGitHub RepositoryからCommand Packをインストールできるようにする。

例:

macos-cli-pack-basic
macos-cli-pack-network
macos-cli-pack-developer

⸻

52. Workflow

将来的に複数コマンドをまとめて実行できるようにする。

例:

Workflow:

Prepare Development Environment

1. caffeinate
2. networksetup
3. launchctl

ただし、MVPでは実装しない。

⸻

53. Natural Language Search

将来的に、

「Macを30分後にスリープさせたい」

などの自然言語からCommandを検索できるようにする。

LLMはCommandの直接実行を担当しない。

LLM:

Natural Language
↓
Candidate Command
↓
Command Definition
↓
Structured Arguments
↓
Validation
↓
User Confirmation
↓
Execution

という安全な構造にする。

LLMから任意shell commandを直接生成して実行してはいけない。

⸻

54. AI Command Explanation

将来的に、

「このコマンドは何をする？」

に対して説明を生成する。

ただし説明生成と実行権限を分離する。

AIには実行権限を与えない。

⸻

55. Project Structure

Phase 1 実装構成:

project-root/
│
├── src/
│   ├── App.tsx
│   ├── main.tsx
│   ├── index.css
│   │
│   ├── components/
│   │   ├── command/
│   │   │   └── CommandDetailView.tsx
│   │   ├── form/
│   │   │   └── DynamicForm.tsx
│   │   ├── result/
│   │   │   └── ExecutionResult.tsx
│   │   └── layout/
│   │       └── Sidebar.tsx
│   │
│   ├── hooks/
│   │   ├── useCommands.ts
│   │   ├── useCommandExecution.ts
│   │   └── useSearch.ts
│   │
│   ├── services/
│   │   └── tauri/
│   │       └── index.ts
│   │
│   ├── types/
│   │   └── index.ts
│   │
│   └── i18n/
│       ├── index.ts
│       └── locales/
│           ├── ja.json
│           └── en.json
│
├── src-tauri/
│   ├── src/
│   │   ├── commands/
│   │   ├── executor/
│   │   ├── builder/
│   │   ├── validator/
│   │   ├── registry/
│   │   ├── storage/
│   │   ├── security/
│   │   ├── native/
│   │   ├── models/
│   │   ├── lib.rs
│   │   └── main.rs
│   │
│   ├── capabilities/
│   ├── icons/
│   ├── Cargo.toml
│   └── tauri.conf.json
│
├── commands/
│   └── power/
│       ├── pmset.json
│       └── caffeinate.json
│
├── docs/
│   └── specification.md
│
├── .gitignore
├── index.html
├── package.json
├── vite.config.ts
├── tsconfig.json
├── tsconfig.app.json
└── tsconfig.node.json

将来的に追加予定:
├── native/
│   └── macos/
│       ├── Package.swift
│       └── Sources/
│           └── MacOSNative/
├── tests/
├── package.json
└── README.md

⸻

56. Rust Module

src-tauri/src/

commands/
→ Tauri IPC command

registry/
→ Command Definition loading

builder/
→ CLI argument generation

validator/
→ Validation

executor/
→ Process execution

parser/
→ man parser（Phase 4）

storage/
→ SQLite

security/
→ security validation

native/
→ Swift Bridge

models/
→ Domain models（CommandDefinition, ArgumentDefinition, ExecutionRecord, DTO）

models/
→ Domain models

⸻

57. Domain Model

CommandDefinition

CommandArgument

CommandCategory

CommandValidationResult

CommandExecution

CommandResult

ExecutionStatus

PermissionRequirement

OSVersionRequirement

⸻

58. Tauri IPC DTO

FrontendとRust間ではDomain Modelを直接公開しすぎない。

DTOを定義する。

例:

CommandSummaryDTO

{
“id”: “…”,
“name”: “…”,
“description”: “…”,
“category”: “…”,
“requiresAdmin”: true
}

CommandDetailDTO

CommandExecutionRequest

CommandExecutionResponse

ValidationResponse

⸻

59. Error Handling

Rust側のエラーをFrontendへ安全なDTOとして返す。

例:

{
“code”: “COMMAND_NOT_FOUND”,
“message”: “Command definition was not found.”
}

内部RustエラーをそのままFrontendへ返さない。

Error Code:

COMMAND_NOT_FOUND
INVALID_ARGUMENT
VALIDATION_FAILED
EXECUTION_FAILED
PERMISSION_DENIED
TIMEOUT
CANCELLED
UNSUPPORTED_OS
BINARY_NOT_FOUND
NATIVE_API_ERROR

⸻

60. Logging

Rust:

* tracing

Frontend:

* development only console logging

Productionでは秘密情報をログへ出力しない。

特に:

* password
* token
* keychain value
* environment variable
* sensitive stdout

をログに出さない。

⸻

61. Binary Discovery

Command Definitionにbinary情報を持たせる。

例:

{
“binary”: {
“name”: “pmset”,
“paths”: [
“/usr/bin/pmset”
]
}
}

実行前にbinary存在を確認する。

PATHに依存しすぎない。

ただしmacOSのバージョンによってパスが変わる可能性があるため、必要に応じて複数候補を許可する。

⸻

62. Environment

CLI実行時のEnvironmentをCommand Definitionから制御可能にする。

ただし任意のEnvironment InjectionはMVPでは許可しない。

基本的に現在のmacOS環境を継承する。

⸻

63. Working Directory

Command Definitionでworking directoryが不要なコマンドは明示的に指定しない。

ファイル操作系Commandのみ必要に応じてユーザーが選択する。

⸻

64. Shell禁止

Command Executorは原則shellを使用しない。

つまり:

Command + argv[]

として実行する。

これによりshell metacharacterによるInjectionリスクを減らす。

⸻

65. macOS Native APIの方針

Swiftを万能Backendとして使わない。

Swift:

macOS-specific API

Rust:

application core

という責務分離を維持する。

macOS CLIを実行するだけならSwiftを経由しない。

⸻

66. Swift Bridge設計

例:

Rust:

NativeService::authorize()

↓

Swift:

authorizePrivilegedOperation()

↓

Result:

Authorized
Denied
Cancelled
Failed

Rust側でDomain Errorへ変換する。

⸻

67. macOS Permissions

以下の権限は必要になった機能ごとに追加する。

* Notifications
* Accessibility
* Keychain
* Administrator authorization
* Login Items

MVPでは不要な権限を要求しない。

⸻

68. App Sandbox

MVP段階でSandbox要件を整理する。

CLI実行やmacOSシステム操作との互換性を確認しながら、必要最小限のentitlement/capabilityを設計する。

Sandboxを理由にセキュリティモデルをFrontendへ移さない。

⸻

69. Build

Development:

vp dev

Tauri development:

tauri dev

Frontend build:

vp build

Tauri production build:

tauri build

実際のpackage scriptはプロジェクトのコマンド体系に合わせて統一する。

例:

vp run tauri:dev
vp run tauri:build

⸻

70. Vite+ Configuration

ルート:

vite.config.ts

で以下を管理:

* Vite
* Vitest
* Oxlint
* Oxfmt
* Vite Task

推奨:

lint.options.typeAware = true

lint.options.typeCheck = true

これによりvp checkをFrontendの標準静的チェックコマンドとする。

⸻

71. Package Scripts

基本:

dev
build
test
check
tauri:dev
tauri:build

ただしVite+のbuilt-in commandと衝突する場合、vp runを利用する。

Vite+ではvp devなどがbuilt-in commandとして予約されるため、package.json scriptを実行する場合はvp run <script>を使用する。

⸻

72. Formatting

TypeScript / TSX / JSON / Markdown:

Oxfmt

Rust:

cargo fmt

Swift:

swift-format

CIではformat違反を検出する。

⸻

73. Lint

TypeScript / React:

Oxlint

React向けルールを有効化する。

Type-aware lintも有効化する。

Rust:

cargo clippy

Swift:

SwiftLintは必要性を見て導入する。

⸻

74. Testing Strategy

Unit

Command Builder
Validator
Search Ranking
Command Parser
DTO conversion

Integration

Command Registry
SQLite
Command Executor

Native

Swift API

E2E

Search
→ Command Detail
→ Form
→ Preview
→ Execute
→ Result

⸻

75. Command Executor Test

実際のmacOSシステムを変更するコマンドをCIで直接実行しない。

Test Commandを用意する。

例:

/usr/bin/printf

または専用mock executableを使用する。

⸻

76. Dangerous Command Test

破壊的コマンドをCIで実行しない。

Command Builder / Validatorだけをテストする。

⸻

77. E2E Test

MVP E2E:

1. アプリ起動
2. Search
3. pmset表示
4. Command Detail
5. Sleep入力
6. Generated Command確認
7. Execute確認画面
8. Cancel

実際のsystem setting変更はE2Eで行わない。

⸻

78. CI

Pull Request:

vp check
vp test
cargo fmt –check
cargo clippy
cargo test
swift test

Main:

全テスト
Tauri build

Release:

Tauri build
Code Signing
Notarization
DMG
GitHub Release

⸻

79. MVP Scope

MVPでは以下だけを完成させる。

* Tauri application
* React UI
* TypeScript
* Rust Core
* Swift Native Layerの基本Bridge
* Command Definition
* Dynamic Form
* Command Search
* Command Builder
* CLI Executor
* stdout / stderr
* exit code
* execution history
* favorites
* pmset
* caffeinate
* networksetup
* scutil
* diskutil
* defaults
* system_profiler
* launchctl
* basic permission handling

⸻

80. MVPでやらないこと

以下はMVPでは実装しない。

* AI
* Natural Language Command
* Linux
* Windows
* Plugin Marketplace
* Community Registry
* Workflow
* Scheduled Command
* 自動man Parser
* Menu Bar
* Spotlight
* 自動アップデート
* 複雑な権限管理
* 全macOS CLI対応

⸻

81. Phase 1

目的:

「GUIからCLIを実行できる最小システム」

実装:

Tauri
React
TypeScript
Vite+
Rust

Command Definition

↓

Dynamic Form

↓

Command Builder

↓

CLI Executor

↓

Result

最初はpmsetだけ対応する。

⸻

82. Phase 2

追加:

* Search
* Categories
* History
* Favorites
* 8 Command対応
* SQLite
* Command Validation

⸻

83. Phase 3

追加:

* Swift Native Layer
* AuthorizationServices
* Keychain
* Notification
* macOS Permission handling

⸻

84. Phase 4

追加:

* man Parser
* Generated Command Definition
* Definition validation
* Definition editor

⸻

85. Phase 5

追加:

* Command Packs
* Plugin architecture
* Community definitions
* Natural language search
* AI explanation

⸻

86. 将来的なAI機能

AIはCommand Executorから完全に分離する。

AI:

「Macのスリープ時間を30分にしたい」

↓

候補:

pmset

↓

Structured arguments:

{
“sleep”: 30
}

↓

Command Definition validation

↓

Generated Command:

pmset -a sleep 30

↓

User confirmation

↓

Rust Executor

この構造を絶対に維持する。

AIが直接shell commandを実行する設計は禁止する。

⸻

87. 将来的なクロスプラットフォーム

CoreのCommand Definitionは将来的なOS拡張を考慮する。

例:

platform:

macos

将来:

linux
windows

OSごとにCommand Definitionを分ける。

例:

commands/
├── macos/
├── linux/
└── windows/

ただしMVPはmacOSのみ。

⸻

88. OSS方針

公開OSSとして開発する。

License:

MITを第一候補とする。

READMEに以下を記載:

* Product Overview
* Features
* Architecture
* Installation
* Development
* Supported macOS Versions
* Security Model
* Contribution
* Command Definition Guide

⸻

89. Contribution

Command Definition追加を簡単にする。

Contributorは:

commands/power/example.json

を追加するだけで基本的なGUI対応ができるようにする。

Pull Requestで自動検証:

* JSON Schema
* command existence
* argument definition
* duplicate ID
* supported OS
* security metadata

をチェックする。

⸻

90. Command Definition Schema

JSON Schemaを提供する。

例:

schemas/command-definition.schema.json

VS Code等でCommand Definitionを編集した際にautocomplete / validationが効くようにする。

⸻

91. Documentation

docs/

├── architecture.md
├── command-definition.md
├── security.md
├── development.md
├── native-api.md
└── contributing.md

を用意する。

⸻

92. Security Principle

最重要原則:

「GUIだから安全」ではなく、「実行可能なCommandを構造的に制限することで安全にする」。

Frontendからの入力はすべてuntrusted inputとして扱う。

Rust側で必ず再検証する。

⸻

93. Execution Pipeline

最終的な実行パイプライン:

React
↓
Tauri IPC
↓
Deserialize
↓
Command ID Lookup
↓
Command Definition Lookup
↓
Argument Validation
↓
Permission Validation
↓
Dangerous Operation Check
↓
Command Builder
↓
Preview
↓
User Confirmation
↓
Native Authorization if required
↓
Process Executor
↓
stdout / stderr
↓
Exit Code
↓
CommandResult
↓
History Repository
↓
React Result UI

⸻

94. 完成形

最終的なアプリケーション:

Mac CLI GUI Manager

Technology:

Tauri 2
+
Vite+
+
React
+
TypeScript
+
Rust
+
Swift

Architecture:

React
→ Tauri
→ Rust
→ macOS CLI

macOS Native:

Rust
→ Swift
→ Apple Framework

Core Concept:

Command Definition
→ Dynamic GUI
→ Safe Command Builder
→ User Confirmation
→ CLI Execution

⸻

95. 開発上の最優先事項

機能数を増やすことよりも、以下を優先する。

1. Command Definitionの設計
2. Rust Coreの責務分離
3. Command Builderの安全性
4. Validation
5. Tauri IPCの境界
6. Permission Architecture
7. Swift Bridgeの責務分離
8. Dynamic Form Generator
9. Testability
10. 将来的なman Parserとの互換性

最初から大量のmacOSコマンドに対応しない。

まず「pmsetをGUI化する」という小さなVertical Sliceを完成させ、その設計が正しいことを確認してからCommand数を増やす。

⸻

96. 最初のVertical Slice

最初に完成させる機能:

React:

Power Management画面

Sleep:
[30] minutes

Display Sleep:
[10] minutes

Power Nap:
[ON]

↓

Generated Command:

pmset -a sleep 30 displaysleep 10 powernap 1

↓

[Execute]

↓

Rust:

Command Definition lookup
↓
Validation
↓
Command Builder
↓
Process Executor

↓

Result:

Exit Code: 0

stdout:
…

↓

SQLite:

Execution History保存

このVertical Sliceを完成させた後、同じCommand Definition方式で他のCLIを追加する。

⸻

97. Definition of Done

MVPは以下をすべて満たした時点で完了とする。

* macOSで起動できる
* React UIが表示される
* Vite+で開発・build・testできる
* vp checkが成功する
* Rust testsが成功する
* Swift testsが成功する
* Command Definitionを読み込める
* Command DefinitionからGUIを自動生成できる
* pmsetをGUIから設定できる
* Generated Commandを確認できる
* shell injectionを許さない
* CLIをRustから直接実行できる
* stdout / stderrを取得できる
* exit codeを取得できる
* 実行履歴を保存できる
* 危険な操作に警告を表示できる
* 権限が必要な操作を適切に扱える
* READMEに開発手順が記載されている
* CIでFrontend / Rust / Swiftのテストが実行される
* 日本語・英語の多言語対応が動作する

⸻

98. 最終方針

このプロジェクトでは、UIフレームワークそのものよりも「Command Definitionを中心にしたCLI abstraction layer」を最重要設計とする。

ReactはPresentation Layer。

RustはApplication / Domain / Execution Layer。

SwiftはmacOS Native Layer。

TauriはDesktop Runtime / IPC Layer。

Vite+はFrontend Toolchain。

各レイヤーの責務を明確に分離する。

特に、Reactから直接OS操作をさせず、すべての実行要求をRustへ渡し、Rust側でCommand Definition・Validation・Securityを通過させてからCLIを実行する。

将来的にAIやman Parserを追加する場合も、最終的な実行経路は必ずCommand Definition → Validation → User Confirmation → Rust Executorとし、AIや外部入力から直接shell commandを実行できない設計を維持する。

⸻

99. 多言語対応（i18n）

Phase 1で実装済み。

使用ライブラリ:

* react-i18next
* i18next
* i18next-browser-languagedetector

対応言語:

* 日本語（ja）- デフォルト
* 英語（en）

翻訳対象:

* アプリ名、サブタイトル
* ホーム画面テキスト
* サイドバー（検索、ローディング、カテゴリ名）
* コマンドDetail（名前、説明、引数名、引数説明）
* 実行結果（完了、失敗、終了コード、出力）
* 履歴画面
* ボタン（検証、プレビュー、実行）
* 単位（minutes→分、seconds→秒）

翻訳管理方針:

* Command Definition JSONのname/descriptionは英語のまま保持
* 画面表示用の翻訳はi18nのJSONファイルで管理
* コマンド名は「日本語名（コマンド名）」の形式で表示
* ユーザーの言語選択はlocalStorageに保存
* ブラウザ言語検出は使用しない（設定上書きの防止）