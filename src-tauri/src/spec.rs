//! コマンド定義の共通インターフェース。
//!
//! 新しいコマンドの追加手順:
//! 1. モジュールを作り `AdminCommand` を実装する (`pmset.rs` を参照)
//! 2. 薄い Tauri エンドポイントを `commands.rs` に追加する
//! 3. `main.rs` の `invoke_handler!` に登録する
//! 4. Frontend に `commands/registry.ts` への登録と対応UIを追加する

use crate::privileged;
use crate::types::CommandResult;

pub trait AdminCommand {
    const ID: &'static str;
    const PROGRAM: &'static str;

    fn args(&self) -> Vec<String>;

    fn preview(&self) -> String {
        format!("sudo {} {}", Self::PROGRAM, self.args().join(" "))
    }

    /// 実行経路の選択 (省略ON/OFF) を含め、定型フローで実行する。
    fn run(&self) -> Result<CommandResult, String> {
        let args = self.args();
        let refs: Vec<&str> = args.iter().map(String::as_str).collect();
        let preview = self.preview();
        if privileged::remember_enabled() {
            privileged::execute_privileged(Self::PROGRAM, &refs, preview)
        } else {
            privileged::execute_with_prompt(Self::PROGRAM, &refs, preview)
        }
    }
}
