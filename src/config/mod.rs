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

/// Plan execution mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PlanExecMode {
    /// Ask for approval before every task.
    #[default]
    Interactive,
    /// Approve the plan once, then run all tasks automatically.
    AutoPlan,
    /// Fully autonomous: no approval gate at all.
    FullAuto,
}

/// Plan mode configuration (`[plan_mode]` section).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanModeConfig {
    #[serde(default)]
    pub mode: PlanExecMode,
    /// Risk score (0–100) above which auto-run pauses for approval.
    #[serde(default = "default_risk_threshold")]
    pub auto_approval_threshold: u8,
    /// Maximum iterations for full-auto mode before forcing a stop.
    #[serde(default = "default_max_iterations")]
    pub max_auto_iterations: u32,
}

fn default_risk_threshold() -> u8 {
    70
}
fn default_max_iterations() -> u32 {
    20
}

/// Security configuration (`[security]` section).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecurityConfig {
    /// Programs permitted in bash tool calls.
    #[serde(default)]
    pub allow_bash: Vec<String>,
    /// Absolute paths that are always denied.
    #[serde(default)]
    pub deny_paths: Vec<String>,
    /// Tool names that are always denied.
    #[serde(default)]
    pub deny_tools: Vec<String>,
}

impl SecurityConfig {
    /// Build a composable permission policy from this security config.
    pub fn to_policy(&self) -> Box<dyn crate::llm::tools::sandbox::PermissionPolicy> {
        use crate::llm::tools::sandbox::{
            AllowAll, AndPolicy, BashCommandAllowlist, DenyPathPrefixRule, DenyToolRule,
        };

        let mut rules: Vec<Box<dyn crate::llm::tools::sandbox::PermissionPolicy>> = Vec::new();

        if !self.allow_bash.is_empty() {
            rules.push(Box::new(BashCommandAllowlist {
                allowed_programs: self.allow_bash.clone(),
            }));
        }

        for tool in &self.deny_tools {
            rules.push(Box::new(DenyToolRule::new(tool)));
        }

        for path in &self.deny_paths {
            rules.push(Box::new(DenyPathPrefixRule::new(path)));
        }

        if rules.is_empty() {
            Box::new(AllowAll)
        } else {
            Box::new(AndPolicy(rules))
        }
    }
}

/// Memory / context configuration (`[memory]` section).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum token budget for injected episodic memories.
    #[serde(default = "default_episodic_budget")]
    pub episodic_budget_tokens: i32,
    /// Whether to build and maintain the codebase symbol index.
    #[serde(default = "default_true")]
    pub enable_codebase_index: bool,
    /// Context compaction threshold (0.0–1.0).
    #[serde(default = "default_compaction_threshold")]
    pub compaction_threshold: f64,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            episodic_budget_tokens: default_episodic_budget(),
            enable_codebase_index: true,
            compaction_threshold: default_compaction_threshold(),
        }
    }
}

fn default_episodic_budget() -> i32 {
    2_000
}
fn default_compaction_threshold() -> f64 {
    0.80
}
fn default_true() -> bool {
    true
}

/// MCP server configuration (`[[mcp.servers]]`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
}

/// Tool result cache configuration (`[tool_cache]` section).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCacheConfig {
    /// TTL in seconds for `read_file` results. 0 = no cache.
    #[serde(default = "default_read_file_ttl")]
    pub read_file_secs: u64,
    /// TTL in seconds for `glob` results.
    #[serde(default = "default_glob_ttl")]
    pub glob_secs: u64,
    /// TTL in seconds for `grep` results.
    #[serde(default = "default_grep_ttl")]
    pub grep_secs: u64,
    /// TTL in seconds for `web_search` results.
    #[serde(default = "default_web_search_ttl")]
    pub web_search_secs: u64,
}

fn default_read_file_ttl() -> u64 {
    60
}
fn default_glob_ttl() -> u64 {
    30
}
fn default_grep_ttl() -> u64 {
    30
}
fn default_web_search_ttl() -> u64 {
    300
}

impl Default for ToolCacheConfig {
    fn default() -> Self {
        Self {
            read_file_secs: default_read_file_ttl(),
            glob_secs: default_glob_ttl(),
            grep_secs: default_grep_ttl(),
            web_search_secs: default_web_search_ttl(),
        }
    }
}

impl ToolCacheConfig {
    /// Return the TTL in seconds for a given tool name. `0` means "do not cache".
    pub fn ttl_secs_for(&self, tool_name: &str) -> u64 {
        match tool_name {
            "read_file" => self.read_file_secs,
            "glob" => self.glob_secs,
            "grep" => self.grep_secs,
            "web_search" => self.web_search_secs,
            _ => 0,
        }
    }
}

/// MCP configuration (`[mcp]` section).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct McpConfig {
    #[serde(default)]
    pub servers: Vec<McpServerConfig>,
}

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

    /// Plan execution mode
    #[serde(default)]
    pub plan_mode: PlanModeConfig,

    /// Security and permission policy
    #[serde(default)]
    pub security: SecurityConfig,

    /// Memory and context management
    #[serde(default)]
    pub memory: MemoryConfig,

    /// MCP server connections
    #[serde(default)]
    pub mcp: McpConfig,

    /// Tool result cache settings
    #[serde(default)]
    pub tool_cache: ToolCacheConfig,
}

/// Debug configuration options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DebugConfig {
    /// Enable LSP debug logging
    #[serde(default)]
    pub debug_lsp: bool,

    /// Enable profiling
    #[serde(default)]
    pub profiling: bool,
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

    /// Qwen/DashScope configuration
    #[serde(default)]
    pub qwen: Option<QwenProviderConfig>,

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

    /// Native Ollama configuration (via `ollama-rs`, `/api/chat`). Distinct
    /// from `providers.openai.base_url` pointed at Ollama's OpenAI-compatible
    /// `/v1` shim - both can be configured, this one unlocks keep_alive,
    /// num_ctx and runtime performance metrics.
    #[serde(default)]
    pub ollama: Option<OllamaProviderConfig>,
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

/// Qwen-specific provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QwenProviderConfig {
    /// Provider enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// API key (for DashScope cloud)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,

    /// API base URL override
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,

    /// Default model to use
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,

    /// Tool call parser: "hermes" or "openai" (default: hermes for local, openai for cloud)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_parser: Option<String>,

    /// Enable Qwen3 thinking mode
    #[serde(default)]
    pub enable_thinking: bool,

    /// Thinking budget tokens (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking_budget: Option<u32>,

    /// DashScope region: "intl" (Singapore) or "cn" (Beijing)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

/// Native Ollama provider configuration (`/api/chat`, via `ollama-rs`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaProviderConfig {
    /// Provider enabled
    #[serde(default = "default_enabled")]
    pub enabled: bool,

    /// Ollama host, e.g. "http://localhost:11434"
    #[serde(default = "default_ollama_host")]
    pub host: String,

    /// Default model to use (e.g. "qwen2.5-coder:7b")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_model: Option<String>,

    /// How long to keep the model loaded in memory: "-1" (indefinitely),
    /// "0" (unload immediately), or a duration like "5m"/"30s"/"2h".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,

    /// Context window size (num_ctx) to request from the model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ctx: Option<u32>,
}

impl Default for OllamaProviderConfig {
    fn default() -> Self {
        Self {
            enabled: default_enabled(),
            host: default_ollama_host(),
            default_model: None,
            keep_alive: None,
            num_ctx: None,
        }
    }
}

fn default_ollama_host() -> String {
    "http://localhost:11434".to_string()
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
            plan_mode: PlanModeConfig::default(),
            security: SecurityConfig::default(),
            memory: MemoryConfig::default(),
            mcp: McpConfig::default(),
            tool_cache: ToolCacheConfig::default(),
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

        // 2. Try to load project-level config (.crustly/config.toml in cwd or ancestors)
        if let Some(project_config) = Self::project_config_path() {
            tracing::debug!("Loading project config from: {:?}", project_config);
            config = Self::merge_from_file(config, &project_config)?;
        }

        // 3. Try to load local config
        let local_config_path = Self::local_config_path();
        if local_config_path.exists() {
            tracing::debug!("Loading local config from: {:?}", local_config_path);
            config = Self::merge_from_file(config, &local_config_path)?;
        }

        // 4. Apply environment variable overrides
        config = Self::apply_env_overrides(config)?;

        tracing::debug!("Configuration loaded successfully");
        Ok(config)
    }

    /// Load configuration from a specific file path
    ///
    /// Priority (lowest to highest):
    /// 1. Default values
    /// 2. Custom config file (specified path)
    /// 3. Environment variables
    pub fn load_from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        tracing::debug!("Loading configuration from custom path: {:?}", path);

        // Start with defaults
        let mut config = Self::default();

        // Load from custom path
        if path.exists() {
            config = Self::merge_from_file(config, path)?;
        } else {
            anyhow::bail!("Config file not found: {:?}", path);
        }

        // Apply environment variable overrides
        config = Self::apply_env_overrides(config)?;

        tracing::debug!("Configuration loaded successfully from custom path");
        Ok(config)
    }

    /// Get the system config path: ~/.config/crustly/config.toml
    fn system_config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|dir| dir.join("crustly").join("config.toml"))
    }

    /// Walk up from cwd looking for `.crustly/config.toml` (project-level config).
    pub fn project_config_path() -> Option<PathBuf> {
        let mut dir = std::env::current_dir().ok()?;
        loop {
            let candidate = dir.join(".crustly").join("config.toml");
            if candidate.exists() {
                return Some(candidate);
            }
            if !dir.pop() {
                return None;
            }
        }
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
        Self {
            crabrace: overlay.crabrace,
            database: overlay.database,
            logging: overlay.logging,
            debug: overlay.debug,
            providers: overlay.providers,
            plan_mode: overlay.plan_mode,
            security: overlay.security,
            memory: overlay.memory,
            mcp: overlay.mcp,
            tool_cache: overlay.tool_cache,
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
            let provider = config.providers.anthropic.get_or_insert(ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        // OpenAI
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            let provider = config.providers.openai.get_or_insert(ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        // OpenAI base URL (for LM Studio, Ollama, etc.)
        if let Ok(base_url) = std::env::var("OPENAI_BASE_URL") {
            let provider = config.providers.openai.get_or_insert(ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.base_url = Some(base_url);
        }

        // Google Gemini
        if let Ok(api_key) = std::env::var("GEMINI_API_KEY") {
            let provider = config.providers.gemini.get_or_insert(ProviderConfig {
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
            let provider = config.providers.azure.get_or_insert(ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.api_key = Some(api_key);
        }

        if let Ok(endpoint) = std::env::var("AZURE_OPENAI_ENDPOINT") {
            let provider = config.providers.azure.get_or_insert(ProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
            });
            provider.base_url = Some(endpoint);
        }

        // Qwen/DashScope
        if let Ok(api_key) = std::env::var("DASHSCOPE_API_KEY") {
            let provider = config.providers.qwen.get_or_insert(QwenProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
                tool_parser: None,
                enable_thinking: false,
                thinking_budget: None,
                region: None,
            });
            provider.api_key = Some(api_key);
        }

        // Qwen base URL (for vLLM, LM Studio, etc.)
        if let Ok(base_url) = std::env::var("QWEN_BASE_URL") {
            let provider = config.providers.qwen.get_or_insert(QwenProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
                tool_parser: None,
                enable_thinking: false,
                thinking_budget: None,
                region: None,
            });
            provider.base_url = Some(base_url);
        }

        // Qwen thinking mode
        if let Ok(thinking) = std::env::var("QWEN_ENABLE_THINKING") {
            let provider = config.providers.qwen.get_or_insert(QwenProviderConfig {
                enabled: true,
                api_key: None,
                base_url: None,
                default_model: None,
                tool_parser: None,
                enable_thinking: false,
                thinking_budget: None,
                region: None,
            });
            provider.enable_thinking = thinking.parse().unwrap_or(false);
        }

        // Ollama native provider (distinct from OPENAI_BASE_URL pointed at
        // Ollama's OpenAI-compatible shim). OLLAMA_HOST matches the official
        // Ollama CLI's own env var convention.
        if let Ok(host) = std::env::var("OLLAMA_HOST").or_else(|_| std::env::var("OLLAMA_BASE_URL"))
        {
            let provider = config
                .providers
                .ollama
                .get_or_insert_with(OllamaProviderConfig::default);
            provider.host = host;
        }

        if let Ok(model) = std::env::var("OLLAMA_MODEL") {
            let provider = config
                .providers
                .ollama
                .get_or_insert_with(OllamaProviderConfig::default);
            provider.default_model = Some(model);
        }

        Ok(())
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<()> {
        tracing::debug!("Validating configuration...");

        // Validate database path parent directory exists
        if let Some(parent) = self.database.path.parent() {
            if !parent.exists() {
                tracing::warn!(
                    "Database parent directory does not exist, will be created: {:?}",
                    parent
                );
            }
        }

        // Validate log level
        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.logging.level.as_str()) {
            anyhow::bail!(
                "Invalid log level: {}. Must be one of: {:?}",
                self.logging.level,
                valid_levels
            );
        }

        // Validate Crabrace URL if enabled
        if self.crabrace.enabled && self.crabrace.base_url.is_empty() {
            anyhow::bail!("Crabrace is enabled but base_url is empty");
        }

        tracing::debug!("Configuration validation passed");
        Ok(())
    }

    /// Save configuration to a file
    pub fn save(&self, path: &Path) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;

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
        assert_eq!(
            config.database.path,
            PathBuf::from("/custom/path/db.sqlite")
        );
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
            config_with_env
                .providers
                .anthropic
                .as_ref()
                .unwrap()
                .api_key,
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
    fn test_ollama_config_from_env() {
        // Both OLLAMA_HOST-precedence and the OLLAMA_BASE_URL fallback are
        // exercised in a single test (rather than two separate #[test] fns)
        // because Rust runs tests in parallel by default and env vars are
        // process-global - two tests toggling OLLAMA_HOST/OLLAMA_BASE_URL
        // concurrently would race each other.
        std::env::remove_var("OLLAMA_HOST");
        std::env::remove_var("OLLAMA_BASE_URL");
        std::env::remove_var("OLLAMA_MODEL");

        std::env::set_var("OLLAMA_HOST", "http://ollama-box:11434");
        std::env::set_var("OLLAMA_MODEL", "qwen2.5-coder:7b");

        let config_with_env = Config::apply_env_overrides(Config::default()).unwrap();
        let ollama = config_with_env
            .providers
            .ollama
            .as_ref()
            .expect("OLLAMA_HOST should populate providers.ollama");
        assert_eq!(ollama.host, "http://ollama-box:11434");
        assert_eq!(ollama.default_model, Some("qwen2.5-coder:7b".to_string()));

        std::env::remove_var("OLLAMA_HOST");
        std::env::remove_var("OLLAMA_MODEL");

        // OLLAMA_BASE_URL is accepted when OLLAMA_HOST isn't set, for
        // consistency with OPENAI_BASE_URL/QWEN_BASE_URL.
        std::env::set_var("OLLAMA_BASE_URL", "http://remote-ollama:11434");
        let config_with_env = Config::apply_env_overrides(Config::default()).unwrap();
        assert_eq!(
            config_with_env.providers.ollama.as_ref().unwrap().host,
            "http://remote-ollama:11434"
        );

        std::env::remove_var("OLLAMA_BASE_URL");
    }

    #[test]
    fn test_ollama_provider_config_default() {
        let cfg = OllamaProviderConfig::default();
        assert!(cfg.enabled);
        assert_eq!(cfg.host, "http://localhost:11434");
        assert_eq!(cfg.default_model, None);
        assert_eq!(cfg.keep_alive, None);
        assert_eq!(cfg.num_ctx, None);
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
