//! Statistics-related Tauri commands

use tauri::State;

use crate::stats::StatsSnapshot;

/// Get statistics snapshot
#[tauri::command]
pub fn get_stats_snapshot(
    proxy_state: State<'_, std::sync::Arc<std::sync::Mutex<crate::commands::proxy::ProxyState>>>,
) -> Result<StatsSnapshot, String> {
    let state = proxy_state.lock().unwrap();
    if let Some(server) = &state.server {
        Ok(server.state().stats.get_snapshot())
    } else {
        Ok(StatsSnapshot::default())
    }
}