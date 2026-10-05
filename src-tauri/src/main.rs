fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            manmen_lib::commands::get_pmset,
            manmen_lib::commands::set_pmset,
            manmen_lib::commands::get_manpage,
            manmen_lib::commands::get_dig,
            manmen_lib::commands::get_ping,
            manmen_lib::commands::get_ssh,
            manmen_lib::commands::start_ssh_tunnel,
            manmen_lib::commands::stop_ssh_tunnel,
            manmen_lib::commands::list_ssh_tunnels,
            manmen_lib::commands::list_ssh_hosts,
            manmen_lib::commands::get_ps,
            manmen_lib::commands::get_lsof,
            manmen_lib::commands::get_top,
            manmen_lib::commands::get_whoami,
            manmen_lib::commands::get_who,
            manmen_lib::commands::get_w,
            manmen_lib::commands::get_scutil,
            manmen_lib::commands::get_git,
            manmen_lib::commands::list_git_repos,
            manmen_lib::commands::get_system_profiler,
            manmen_lib::commands::prune_docker_builder,
            manmen_lib::commands::docker_builder_du,
            manmen_lib::commands::docker_builder_ls,
            manmen_lib::commands::docker_builder_inspect,
            manmen_lib::commands::docker_builder_version,
            manmen_lib::commands::docker_containers,
            manmen_lib::commands::docker_images,
            manmen_lib::commands::docker_networks,
            manmen_lib::commands::docker_volumes,
            manmen_lib::commands::docker_system_df,
            manmen_lib::commands::docker_system_info,
            manmen_lib::commands::set_remember,
        ])
        // ウィンドウを閉じたらSSHトンネルを閉じ残さない
        .on_window_event(|_window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                manmen_lib::ssh::kill_all_tunnels();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
