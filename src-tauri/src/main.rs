//! AIProxy - AI Model Request Proxy with Auto-Retry
//!
//! A desktop application that acts as a reverse proxy for AI model APIs,
//! automatically retrying failed requests to prevent task interruptions.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use ai_proxy_lib::{commands::ProxyState, config::ConfigManager, tray};

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
            ai_proxy_lib::commands::get_config,
            ai_proxy_lib::commands::save_config,
            ai_proxy_lib::commands::create_profile,
            ai_proxy_lib::commands::update_profile,
            ai_proxy_lib::commands::delete_profile,
            ai_proxy_lib::commands::set_active_profile,
            ai_proxy_lib::commands::start_proxy,
            ai_proxy_lib::commands::stop_proxy,
            ai_proxy_lib::commands::get_proxy_status,
            ai_proxy_lib::commands::get_stats,
            ai_proxy_lib::commands::reset_stats,
        ])
        .setup(|app| {
            // Initialize system tray
            tray::setup_tray(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
