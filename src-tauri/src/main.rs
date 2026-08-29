use std::sync::Mutex;
use std::path::PathBuf;

use manmen_lib::registry::CommandRegistry;
use manmen_lib::storage::Storage;
use manmen_lib::commands::AppState;

fn main() {
    env_logger::init();

    let mut registry = CommandRegistry::new();

    // Load command definitions from commands/ directory
    let commands_dir = get_commands_dir();
    if let Err(e) = registry.load_from_dir(&commands_dir) {
        log::error!("Failed to load commands: {}", e);
    }

    // Initialize storage
    let db_path = get_db_path();
    let storage = Storage::new(&db_path).expect("Failed to initialize storage");

    let state = AppState {
        registry: Mutex::new(registry),
        storage: Mutex::new(storage),
    };

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            manmen_lib::commands::get_commands,
            manmen_lib::commands::get_command,
            manmen_lib::commands::search_commands,
            manmen_lib::commands::get_categories,
            manmen_lib::commands::validate_command,
            manmen_lib::commands::build_command,
            manmen_lib::commands::execute_command,
            manmen_lib::commands::get_execution_history,
            manmen_lib::commands::get_execution,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_commands_dir() -> PathBuf {
    // In development, use the commands directory relative to the project root
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .join("commands")
}

fn get_db_path() -> PathBuf {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("manmen");

    std::fs::create_dir_all(&data_dir).ok();

    data_dir.join("manmen.db")
}
