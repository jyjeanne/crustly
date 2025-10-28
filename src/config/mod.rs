//! Configuration Module
//!
//! Handles application configuration loading, validation, and management.

pub mod crabrace;
pub mod secrets;
pub mod update;

pub use crabrace::{CrabraceConfig, CrabraceIntegration};
pub use secrets::{ProviderSecrets, SecretString};
pub use update::{ProviderUpdater, UpdateResult};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

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

    /// Debug options
    #[serde(default)]
    pub debug: DebugConfig,

    /// LLM provider configurations
    #[serde(default)]
    pub providers: ProviderConfigs,
}

/// Debug configuration options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    /// Enable LSP debug logging
    #[serde(default)]
    pub debug_lsp: bool,

    /// Enable profiling
    #[serde(default)]
    pub profiling: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            debug_lsp: false,
            profiling: false,
        }
    }
}

/// LLM Provider configurations
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProviderConfigs {
    /// Anthropic configuration
    #[serde(default)]
    pub anthropic: Option<ProviderConfig>,

    /// OpenAI configuration
    #[serde(default)]
    pub openai: Option<ProviderConfig>,

    /// Google Gemini configuration
    #[serde(default)]
    pub gemini: Option<ProviderConfig>,

    /// AWS Bedrock configuration
    #[serde(default)]
    pub bedrock: Option<ProviderConfig>,

    /// Azure OpenAI configuration
    #[serde(default)]
    pub azure: Option<ProviderConfig>,

    /// VertexAI configuration
    #[serde(default)]
    pub vertex: Option<ProviderConfig>,
}

/// Individual provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Provider enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// API key (will be loaded from env or secrets)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// API base URL override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Default model to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// Path to SQLite database file
    #[serde(default = "default_db_path")]
    pub path: PathBuf,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            path: default_db_path(),
        }
    }
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

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: default_log_level(),
            file: None,
        }
    }
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
            debug: DebugConfig::default(),
            providers: ProviderConfigs::default(),
        }
    }
}

impl Config {
    /// Load configuration from default locations
    ///
    /// Priority (lowest to highest):
    /// 1. Default values
    /// 2. System config: ~/.config/crustly/config.toml
    /// 3. Local config: ./crustly.toml
    /// 4. Environment variables
    pub fn load() -> Result<Self> {
        tracing::debug!("Loading configuration...");

        // Start with defaults
        let mut config = Self::default();

        // 1. Try to load system config
        if let Some(system_config_path) = Self::system_config_path() {
            if system_config_path.exists() {
                tracing::debug!("Loading system config from: {:?}", system_config_path);
                config = Self::merge_from_file(config, &system_config_path)?;
            }
        }

        // 2. Try to load local config
        let local_config_path = Self::local_config_path();
        if local_config_path.exists() {
            tracing::debug!("Loading local config from: {:?}", local_config_path);
            config = Self::merge_from_file(config, &local_config_path)?;
        }

        // 3. Apply environment variable overrides
        config = Self::apply_env_overrides(config)?;

        tracing::debug!("Configuration loaded successfully");
        Ok(config)
    }

    /// Get the system config path: ~/.config/crustly/config.toml
    fn system_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("crustly").join("config.toml"))
    }

    /// Get the local config path: ./crustly.toml
    fn local_config_path() -> PathBuf {
        PathBuf::from("./crustly.toml")
    }

    /// Load and merge configuration from a TOML file
    fn merge_from_file(base: Self, path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("Failed to read config file: {:?}", path))?;

        let file_config: Self = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {:?}", path))?;

        Ok(Self::merge(base, file_config))
    }

    /// Merge two configs (file_config overwrites base where specified)
    fn merge(_base: Self, overlay: Self) -> Self {
        // For now, we'll do a simple overlay merge where overlay completely replaces base
        // In the future, we could make this more sophisticated with field-level merging
        Self {
            crabrace: overlay.crabrace,
            database: overlay.database,
            logging: overlay.logging,
            debug: overlay.debug,
            providers: overlay.providers,
        }
    }

    /// Apply environment variable overrides
    fn apply_env_overrides(mut config: Self) -> Result<Self> {
        // Database path
        if let Ok(db_path) = std::env::var("CRUSTLY_DB_PATH") {
            config.database.path = PathBuf::from(db_path);
        }

        // Log level
        if let Ok(log_level) = std::env::var("CRUSTLY_LOG_LEVEL") {
            config.logging.level = log_level;
        }

        // Log file
        if let Ok(log_file) = std::env::var("CRUSTLY_LOG_FILE") {
            config.logging.file = Some(PathBuf::from(log_file));
        }

        // Debug options
        if let Ok(debug_lsp) = std::env::var("CRUSTLY_DEBUG_LSP") {
            config.debug.debug_lsp = debug_lsp.parse().unwrap_or(false);
        }

        if let Ok(profiling) = std::env::var("CRUSTLY_PROFILING") {
            config.debug.profiling = profiling.parse().unwrap_or(false);
        }

        // Crabrace options
        if let Ok(enabled) = std::env::var("CRUSTLY_CRABRACE_ENABLED") {
            config.crabrace.enabled = enabled.parse().unwrap_or(true);
        }

        if let Ok(base_url) = std::env::var("CRUSTLY_CRABRACE_URL") {
            config.crabrace.base_url = base_url;
        }

        if let Ok(auto_update) = std::env::var("CRUSTLY_CRABRACE_AUTO_UPDATE") {
            config.crabrace.auto_update = auto_update.parse().unwrap_or(true);
        }

        // Provider API keys from environment
        Self::load_provider_api_keys(&mut config)?;

        Ok(config)
    }

    /// Load provider API keys from environment variables
    fn load_provider_api_keys(config: &mut Self) -> Result<()> {
        // Anthropic
        if let Ok(api_key) = std::env::var("ANTHROPIC_API_KEY") {
            let provider = config.providers.anthropic.get_or_insert_with(|| ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        // OpenAI
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            let provider = config.providers.openai.get_or_insert_with(|| ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        // Google Gemini
        if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
            let provider = config.providers.gemini.get_or_insert_with(|| ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        // AWS credentials are typically loaded via AWS SDK default chain
        // Azure uses AZURE_OPENAI_KEY and AZURE_OPENAI_ENDPOINT
        if let Ok(api_key) = std::env::var("AZURE_OPENAI_KEY") {
            let provider = config.providers.azure.get_or_insert_with(|| ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        if let Ok(endpoint) = std::env::var("AZURE_OPENAI_ENDPOINT") {
            let provider = config.providers.azure.get_or_insert_with(|| ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.base_url = Some(endpoint);
        }

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        tracing::debug!("Validating configuration...");

        // Validate database path parent directory exists
        if let Some(parent) = self.database.path.parent() {
            if !parent.exists() {
                tracing::warn!("Database parent directory does not exist, will be created: {:?}", parent);
            }
        }

        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            anyhow::bail!("Invalid log level: {}. Must be one of: {:?}", self.logging.level, valid_levels);
        }

        // Validate Crabrace URL if enabled
        if self.crabrace.enabled {
            if self.crabrace.base_url.is_empty() {
                anyhow::bail!("Crabrace is enabled but base_url is empty");
            }
        }

        tracing::debug!("Configuration validation passed");
        Ok(())
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML")?;

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {:?}", parent))?;
        }

        fs::write(path, toml_string)
            .with_context(|| format!("Failed to write config file: {:?}", path))?;

        tracing::info!("Configuration saved to: {:?}", path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.crabrace.enabled);
        assert_eq!(config.logging.level, "info");
        assert!(!config.debug.debug_lsp);
        assert!(!config.debug.profiling);
    }

    #[test]
    fn test_config_validation() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_config_validation_invalid_log_level() {
        let mut config = Config::default();
        config.logging.level = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_validation_empty_crabrace_url() {
        let mut config = Config::default();
        config.crabrace.base_url = String::new();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_config_from_toml() {
        let toml_content = r#"
[database]
path = "/custom/path/db.sqlite"

[logging]
level = "debug"

[debug]
debug_lsp = true
profiling = true

[crabrace]
enabled = false
        "#;

        let config: Config = toml::from_str(toml_content).unwrap();
        assert_eq!(config.database.path, PathBuf::from("/custom/path/db.sqlite"));
        assert_eq!(config.logging.level, "debug");
        assert!(config.debug.debug_lsp);
        assert!(config.debug.profiling);
        assert!(!config.crabrace.enabled);
    }

    #[test]
    fn test_config_save_and_load() {
        let temp_file = NamedTempFile::new().unwrap();
        let config = Config::default();

        // Save config
        config.save(temp_file.path()).unwrap();

        // Load config back
        let contents = std::fs::read_to_string(temp_file.path()).unwrap();
        let loaded_config: Config = toml::from_str(&contents).unwrap();

        assert_eq!(loaded_config.logging.level, config.logging.level);
        assert_eq!(loaded_config.crabrace.enabled, config.crabrace.enabled);
    }

    #[test]
    fn test_config_env_overrides() {
        // Set environment variables
        std::env::set_var("CRUSTLY_LOG_LEVEL", "trace");
        std::env::set_var("CRUSTLY_DEBUG_LSP", "true");
        std::env::set_var("CRUSTLY_PROFILING", "true");
        std::env::set_var("CRUSTLY_DB_PATH", "/tmp/test.db");

        let config = Config::default();
        let config_with_env = Config::apply_env_overrides(config).unwrap();

        assert_eq!(config_with_env.logging.level, "trace");
        assert!(config_with_env.debug.debug_lsp);
        assert!(config_with_env.debug.profiling);
        assert_eq!(config_with_env.database.path, PathBuf::from("/tmp/test.db"));

        // Clean up
        std::env::remove_var("CRUSTLY_LOG_LEVEL");
        std::env::remove_var("CRUSTLY_DEBUG_LSP");
        std::env::remove_var("CRUSTLY_PROFILING");
        std::env::remove_var("CRUSTLY_DB_PATH");
    }

    #[test]
    fn test_provider_config_api_keys_from_env() {
        // Set environment variables
        std::env::set_var("ANTHROPIC_API_KEY", "test-anthropic-key");
        std::env::set_var("OPENAI_API_KEY", "test-openai-key");

        let config = Config::default();
        let config_with_env = Config::apply_env_overrides(config).unwrap();

        assert!(config_with_env.providers.anthropic.is_some());
        assert_eq!(
            config_with_env.providers.anthropic.as_ref().unwrap().api_key,
            Some("test-anthropic-key".to_string())
        );

        assert!(config_with_env.providers.openai.is_some());
        assert_eq!(
            config_with_env.providers.openai.as_ref().unwrap().api_key,
            Some("test-openai-key".to_string())
        );

        // Clean up
        std::env::remove_var("ANTHROPIC_API_KEY");
        std::env::remove_var("OPENAI_API_KEY");
    }

    #[test]
    fn test_system_config_path() {
        let path = Config::system_config_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.to_string_lossy().contains("crustly"));
        assert!(path.to_string_lossy().ends_with("config.toml"));
    }

    #[test]
    fn test_local_config_path() {
        let path = Config::local_config_path();
        assert_eq!(path, PathBuf::from("./crustly.toml"));
    }

    #[test]
    fn test_debug_config_default() {
        let debug = DebugConfig::default();
        assert!(!debug.debug_lsp);
        assert!(!debug.profiling);
    }

    #[test]
    fn test_provider_configs_default() {
        let providers = ProviderConfigs::default();
        assert!(providers.anthropic.is_none());
        assert!(providers.openai.is_none());
        assert!(providers.gemini.is_none());
        assert!(providers.bedrock.is_none());
        assert!(providers.azure.is_none());
        assert!(providers.vertex.is_none());
    }

    #[test]
    fn test_database_config_default() {
        let db_config = DatabaseConfig::default();
        assert!(!db_config.path.as_os_str().is_empty());
    }

    #[test]
    fn test_logging_config_default() {
        let logging = LoggingConfig::default();
        assert_eq!(logging.level, "info");
        assert!(logging.file.is_none());
    }
}
