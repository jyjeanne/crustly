//! Configuration Module
//!
//! Handles application configuration loading, validation, and management.

pub mod crabrace;

pub use crabrace::{CrabraceConfig, CrabraceIntegration};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Crabrace integration configuration
    #[serde(default)]
    pub crabrace: CrabraceConfig,

    /// Database configuration
    #[serde(default)]
    pub database: DatabaseConfig,

    /// Logging configuration
    #[serde(default)]
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Path to SQLite database file
    #[serde(default = "default_db_path")]
    pub path: PathBuf,
}

fn default_db_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("crustly")
        .join("crustly.db")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub level: String,

    /// Log to file
    #[serde(default)]
    pub file: Option<PathBuf>,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            crabrace: CrabraceConfig::default(),
            database: DatabaseConfig {
                path: default_db_path(),
            },
            logging: LoggingConfig {
                level: default_log_level(),
                file: None,
            },
        }
    }
}

impl Config {
    /// Load configuration from default locations
    pub fn load() -> Result<Self> {
        // TODO: Implement config file loading from:
        // - ~/.config/crustly/config.toml
        // - ./crustly.toml
        // - Environment variables
        tracing::debug!("Loading configuration...");
        Ok(Self::default())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        tracing::debug!("Validating configuration...");
        // TODO: Implement validation logic
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.crabrace.enabled);
        assert_eq!(config.logging.level, "info");
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }
}
