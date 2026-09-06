fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            manmen_lib::commands::get_disablesleep,
            manmen_lib::commands::set_disablesleep,
            manmen_lib::commands::set_remember,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
