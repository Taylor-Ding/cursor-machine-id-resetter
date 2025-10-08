// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod core;
mod utils;


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_machine_info,
            commands::reset_machine_id,
            commands::check_cursor_status,
            commands::quit_cursor,
            commands::get_backups,
            commands::restore_from_backup,
            commands::delete_backup,
            commands::get_settings,
            commands::save_settings,
            commands::get_default_settings,
            commands::generate_account,
            commands::restore_file_backup,
        ])
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

