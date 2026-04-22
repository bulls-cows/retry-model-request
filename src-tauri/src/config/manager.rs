//! Configuration manager for persistent storage

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

use super::{Config, ProxyProfile};

/// Manages application configuration with persistence
pub struct ConfigManager {
    config: Mutex<Config>,
    config_path: PathBuf,
}

impl ConfigManager {
    /// Create a new configuration manager
    pub fn new() -> anyhow::Result<Self> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("retry-model-request");

        // Ensure config directory exists
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)?;
        }

        let config_path = config_dir.join("config.json");

        let config = if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            Config::default()
        };

        Ok(Self {
            config: Mutex::new(config),
            config_path,
        })
    }

    /// Get the current configuration
    pub fn get_config(&self) -> Config {
        self.config.lock().unwrap().clone()
    }

    /// Save configuration to disk
    pub fn save_config(&self, config: &Config) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_path, content)?;
        *self.config.lock().unwrap() = config.clone();
        Ok(())
    }

    /// Get the active profile
    pub fn get_active_profile(&self) -> Option<ProxyProfile> {
        self.config.lock().unwrap().active_profile().cloned()
    }

    /// Update a profile by ID
    pub fn update_profile(&self, profile: ProxyProfile) -> anyhow::Result<()> {
        let mut config = self.config.lock().unwrap();
        if let Some(existing) = config.profiles.iter_mut().find(|p| p.id == profile.id) {
            *existing = profile;
        } else {
            config.profiles.push(profile);
        }
        drop(config);
        self.save_config(&self.get_config())
    }

    /// Create a new profile
    pub fn create_profile(&self, name: String) -> anyhow::Result<ProxyProfile> {
        let profile = ProxyProfile {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            ..Default::default()
        };
        let mut config = self.config.lock().unwrap();
        config.profiles.push(profile.clone());
        drop(config);
        self.save_config(&self.get_config())?;
        Ok(profile)
    }

    /// Delete a profile by ID
    pub fn delete_profile(&self, id: &str) -> anyhow::Result<()> {
        let mut config = self.config.lock().unwrap();
        config.profiles.retain(|p| p.id != id);
        if config.active_profile_id.as_deref() == Some(id) {
            config.active_profile_id = config.profiles.first().map(|p| p.id.clone());
        }
        drop(config);
        self.save_config(&self.get_config())
    }

    /// Set the active profile
    pub fn set_active_profile(&self, id: &str) -> anyhow::Result<()> {
        let mut config = self.config.lock().unwrap();
        if config.profiles.iter().any(|p| p.id == id) {
            config.active_profile_id = Some(id.to_string());
        }
        drop(config);
        self.save_config(&self.get_config())
    }
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new().expect("Failed to initialize ConfigManager")
    }
}
