fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            manmen_lib::commands::get_pmset,
            manmen_lib::commands::set_pmset,
            manmen_lib::commands::get_manpage,
            manmen_lib::commands::get_top,
            manmen_lib::commands::set_remember,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
