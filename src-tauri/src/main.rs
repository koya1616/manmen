fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            manmen_lib::commands::get_pmset,
            manmen_lib::commands::set_pmset,
            manmen_lib::commands::get_manpage,
            manmen_lib::commands::get_top,
            manmen_lib::commands::prune_docker_builder,
            manmen_lib::commands::docker_builder_du,
            manmen_lib::commands::docker_builder_ls,
            manmen_lib::commands::docker_builder_inspect,
            manmen_lib::commands::docker_builder_version,
            manmen_lib::commands::set_remember,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
