//! Retry Model Request - AI Model Request Proxy with Auto-Retry
//!
//! A desktop application that acts as a reverse proxy for AI model APIs,
//! automatically retrying failed requests to prevent task interruptions.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use retry_model_request_lib::{
    commands::{
        create_profile, delete_profile, get_config, get_proxy_status, get_stats, reset_stats,
        save_config, set_active_profile, start_proxy, stop_proxy, update_profile, ProxyState,
    },
    config::ConfigManager,
    tray,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .manage(ConfigManager::new().expect("Failed to initialize config"))
        .manage(Arc::new(std::sync::Mutex::new(ProxyState::default())))
        .invoke_handler(tauri::generate_handler![
            get_config,
            save_config,
            create_profile,
            update_profile,
            delete_profile,
            set_active_profile,
            start_proxy,
            stop_proxy,
            get_proxy_status,
            get_stats,
            reset_stats,
        ])
        .setup(|app| {
            // Initialize system tray
            tray::setup_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
