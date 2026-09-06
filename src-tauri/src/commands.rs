//! Tauri IPC エンドポイント。薄く保つこと。
//! 実処理は各コマンドモジュール (`pmset.rs`・`manpage.rs` 等) と共通基盤 (`privileged.rs`) に置く。

use crate::pmset::DisablesleepState;
use crate::types::CommandResult;

#[tauri::command]
pub fn get_disablesleep() -> Result<DisablesleepState, String> {
    crate::pmset::current()
}

#[tauri::command]
pub fn set_disablesleep(enabled: bool) -> Result<CommandResult, String> {
    crate::pmset::apply(enabled)
}

#[tauri::command]
pub fn get_manpage(topic: String) -> Result<CommandResult, String> {
    crate::manpage::Manpage::new(&topic).and_then(|cmd| cmd.run())
}

#[tauri::command]
pub fn set_remember(enabled: bool) {
    crate::privileged::set_remember(enabled);
}
