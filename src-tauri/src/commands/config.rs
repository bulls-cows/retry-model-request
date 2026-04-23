//! Configuration-related Tauri commands

use tauri::State;

use crate::config::{Config, ConfigManager, ProxyProfile};

/// Get current configuration
#[tauri::command]
pub fn get_config(manager: State<'_, ConfigManager>) -> Config {
    manager.get_config()
}

/// Save configuration
#[tauri::command]
pub fn save_config(manager: State<'_, ConfigManager>, config: Config) -> Result<(), String> {
    manager.save_config(&config).map_err(|e| e.to_string())
}

/// Create a new profile
#[tauri::command]
pub fn create_profile(
    manager: State<'_, ConfigManager>,
    name: String,
) -> Result<ProxyProfile, String> {
    manager.create_profile(name).map_err(|e| e.to_string())
}

/// Update a profile
#[tauri::command]
pub fn update_profile(
    manager: State<'_, ConfigManager>,
    profile: ProxyProfile,
) -> Result<(), String> {
    manager.update_profile(profile).map_err(|e| e.to_string())
}

/// Delete a profile
#[tauri::command]
pub fn delete_profile(manager: State<'_, ConfigManager>, id: String) -> Result<(), String> {
    manager.delete_profile(&id).map_err(|e| e.to_string())
}

/// Set active profile
#[tauri::command]
pub fn set_active_profile(manager: State<'_, ConfigManager>, id: String) -> Result<(), String> {
    manager.set_active_profile(&id).map_err(|e| e.to_string())
}
