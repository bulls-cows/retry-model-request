//! Proxy-related Tauri commands

use std::sync::Arc;
use tauri::{AppHandle, Emitter, State};

use crate::config::ConfigManager;
use crate::proxy::{LogEntry, ProxyServer};
use crate::stats::StatsSnapshot;

/// Global proxy server state
pub struct ProxyState {
    server: Option<ProxyServer>,
}

impl Default for ProxyState {
    fn default() -> Self {
        Self { server: None }
    }
}

/// Start the proxy server
#[tauri::command]
pub async fn start_proxy(
    app: AppHandle,
    config_manager: State<ConfigManager>,
    proxy_state: State<'_, Arc<std::sync::Mutex<ProxyState>>>,
) -> Result<u16, String> {
    let profile = config_manager
        .get_active_profile()
        .ok_or_else(|| "No active profile".to_string())?;

    if profile.target_base_url.is_empty() {
        return Err("Target URL is not configured".to_string());
    }

    let mut state = proxy_state.lock().unwrap();

    // Stop existing server if running
    if let Some(mut server) = state.server.take() {
        server.stop();
    }

    // Create and start new server
    let (mut server, mut log_rx) = ProxyServer::new(profile);
    let port = server.start().await.map_err(|e| e.to_string())?;

    // Spawn log forwarding task
    let app_handle = app.clone();
    tokio::spawn(async move {
        while let Ok(log) = log_rx.recv().await {
            app_handle.emit("proxy-log", log).ok();
        }
    });

    // Spawn stats forwarding task
    let stats = server.state().stats.clone();
    let app_handle = app.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            let snapshot = stats.get_snapshot();
            app_handle.emit("proxy-stats", snapshot).ok();
        }
    });

    state.server = Some(server);

    // Emit status event
    app.emit("proxy-status", "running").ok();

    Ok(port)
}

/// Stop the proxy server
#[tauri::command]
pub fn stop_proxy(
    app: AppHandle,
    proxy_state: State<'_, Arc<std::sync::Mutex<ProxyState>>>,
) -> Result<(), String> {
    let mut state = proxy_state.lock().unwrap();

    if let Some(mut server) = state.server.take() {
        server.stop();
    }

    app.emit("proxy-status", "stopped").ok();

    Ok(())
}

/// Get proxy status
#[tauri::command]
pub fn get_proxy_status(
    proxy_state: State<'_, Arc<std::sync::Mutex<ProxyState>>>,
) -> Result<String, String> {
    let state = proxy_state.lock().unwrap();
    Ok(if state.server.is_some() { "running" } else { "stopped" })
}

/// Get current statistics
#[tauri::command]
pub fn get_stats(
    proxy_state: State<'_, Arc<std::sync::Mutex<ProxyState>>>,
) -> Result<StatsSnapshot, String> {
    let state = proxy_state.lock().unwrap();
    if let Some(server) = &state.server {
        Ok(server.state().stats.get_snapshot())
    } else {
        Ok(StatsSnapshot {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            total_retries: 0,
            success_rate: 0.0,
        })
    }
}

/// Reset statistics
#[tauri::command]
pub fn reset_stats(
    proxy_state: State<'_, Arc<std::sync::Mutex<ProxyState>>>,
) -> Result<(), String> {
    let state = proxy_state.lock().unwrap();
    if let Some(server) = &state.server {
        server.state().stats.reset();
    }
    Ok(())
}