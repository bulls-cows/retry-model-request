//! Configuration schema definitions

use serde::{Deserialize, Serialize};

/// Proxy profile containing all settings for a specific target
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyProfile {
    /// Unique identifier for the profile
    pub id: String,
    /// Display name for the profile
    pub name: String,
    /// Local port to listen on
    pub local_port: u16,
    /// Target base URL to proxy requests to
    pub target_base_url: String,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Delay between retries in milliseconds
    pub retry_delay_ms: u64,
    /// HTTP status codes that should trigger a retry
    pub retry_status_codes: Vec<u16>,
}

impl Default for ProxyProfile {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "Default".to_string(),
            local_port: 3000,
            target_base_url: String::new(),
            max_retries: 3,
            retry_delay_ms: 1000,
            retry_status_codes: vec![429, 500, 502, 503, 504],
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// List of saved proxy profiles
    pub profiles: Vec<ProxyProfile>,
    /// ID of the currently active profile
    pub active_profile_id: Option<String>,
    /// Whether to start the proxy automatically on app launch
    pub auto_start: bool,
    /// Whether to minimize to system tray on close
    pub minimize_to_tray: bool,
    /// Whether to start on system boot
    pub start_on_boot: bool,
}

impl Default for Config {
    fn default() -> Self {
        let default_profile = ProxyProfile::default();
        Self {
            profiles: vec![default_profile.clone()],
            active_profile_id: Some(default_profile.id),
            auto_start: false,
            minimize_to_tray: true,
            start_on_boot: false,
        }
    }
}

impl Config {
    /// Get the currently active profile
    pub fn active_profile(&self) -> Option<&ProxyProfile> {
        self.active_profile_id
            .as_ref()
            .and_then(|id| self.profiles.iter().find(|p| &p.id == id))
    }

    /// Get mutable reference to the active profile
    pub fn active_profile_mut(&mut self) -> Option<&mut ProxyProfile> {
        if let Some(id) = self.active_profile_id.clone() {
            self.profiles.iter_mut().find(|p| p.id == id)
        } else {
            None
        }
    }
}
