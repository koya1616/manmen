//! Tauri IPC エンドポイント。薄く保つこと。
//! 実処理は各コマンドモジュール (`pmset.rs`・`manpage.rs` 等) と共通基盤 (`privileged.rs`) に置く。

use crate::manpage::ManpageDocument;
use crate::pmset::PmsetState;
use crate::top::TopSnapshot;
use crate::types::CommandResult;

#[tauri::command]
pub fn get_pmset() -> Result<PmsetState, String> {
    crate::pmset::current()
}

#[tauri::command]
pub fn set_pmset(scope: String, setting: String, value: String) -> Result<CommandResult, String> {
    crate::pmset::apply(scope, setting, value)
}

#[tauri::command]
pub fn get_manpage(topic: String) -> Result<ManpageDocument, String> {
    crate::manpage::Manpage::new(&topic).and_then(|cmd| cmd.run())
}

#[tauri::command]
pub fn get_top(sort_key: String, count: u32) -> Result<TopSnapshot, String> {
    crate::top::get_snapshot(sort_key, count)
}

#[tauri::command]
pub fn set_remember(enabled: bool) {
    crate::privileged::set_remember(enabled);
}
