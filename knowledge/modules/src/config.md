---
type: Rust Module
title: config
resource: src/config/mod.rs#L1-L1593
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/pub-use-crabrace-crabraceconfig-crabraceintegration
  - external/pub-use-secrets-providersecrets-secretstring
  - external/pub-use-update-providerupdater-updateresult
  - external/anyhow-context-result
  - external/serde-deserialize-serialize
  - external/std-fs
  - external/std-path-path-pathbuf
  - external/crate-llm-tools-sandbox-allowall-andpolicy-bashcommandallowlist-denypathprefixrule-denytoolrule
  - external/super
  - external/tempfile-namedtempfile
  - external/crate-llm-tools-sandbox-policydecision
  member_of:
  - packages/crustly
---

# Contains

- [PlanExecMode](../../classes/src/config/PlanExecMode.md)
- [PlanModeConfig](../../classes/src/config/PlanModeConfig.md)
- [default_risk_threshold](../../functions/src/config/default_risk_threshold.md)
- [default_max_iterations](../../functions/src/config/default_max_iterations.md)
- [SecurityConfig](../../classes/src/config/SecurityConfig.md)
- [to_policy](../../functions/src/config/SecurityConfig/to_policy.md)
- [MemoryConfig](../../classes/src/config/MemoryConfig.md)
- [default](../../functions/src/config/MemoryConfig/default/default.md)
- [default_episodic_budget](../../functions/src/config/default_episodic_budget.md)
- [default_compaction_threshold](../../functions/src/config/default_compaction_threshold.md)
- [default_true](../../functions/src/config/default_true.md)
- [McpServerConfig](../../classes/src/config/McpServerConfig.md)
- [ToolCacheConfig](../../classes/src/config/ToolCacheConfig.md)
- [default_read_file_ttl](../../functions/src/config/default_read_file_ttl.md)
- [default_glob_ttl](../../functions/src/config/default_glob_ttl.md)
- [default_grep_ttl](../../functions/src/config/default_grep_ttl.md)
- [default_web_search_ttl](../../functions/src/config/default_web_search_ttl.md)
- [default](../../functions/src/config/ToolCacheConfig/default/default.md)
- [ttl_secs_for](../../functions/src/config/ToolCacheConfig/ttl_secs_for.md)
- [McpConfig](../../classes/src/config/McpConfig.md)
- [Config](../../classes/src/config/Config.md)
- [DebugConfig](../../classes/src/config/DebugConfig.md)
- [ProviderConfigs](../../classes/src/config/ProviderConfigs.md)
- [override_default_model](../../functions/src/config/ProviderConfigs/override_default_model.md)
- [llama_cpp_models_dir](../../functions/src/config/ProviderConfigs/llama_cpp_models_dir.md)
- [ProviderConfig](../../classes/src/config/ProviderConfig.md)
- [QwenProviderConfig](../../classes/src/config/QwenProviderConfig.md)
- [default](../../functions/src/config/QwenProviderConfig/default/default.md)
- [OllamaProviderConfig](../../classes/src/config/OllamaProviderConfig.md)
- [OllamaModelConfig](../../classes/src/config/OllamaModelConfig.md)
- [ThinkSetting](../../classes/src/config/ThinkSetting.md)
- [as_str](../../functions/src/config/ThinkSetting/as_str.md)
- [LlamaCppProviderConfig](../../classes/src/config/LlamaCppProviderConfig.md)
- [default](../../functions/src/config/LlamaCppProviderConfig/default/default.md)
- [default_llama_cpp_n_ctx](../../functions/src/config/default_llama_cpp_n_ctx.md)
- [default](../../functions/src/config/OllamaProviderConfig/default/default.md)
- [default_ollama_host](../../functions/src/config/default_ollama_host.md)
- [default_enabled](../../functions/src/config/default_enabled.md)
- [DatabaseConfig](../../classes/src/config/DatabaseConfig.md)
- [default](../../functions/src/config/DatabaseConfig/default/default.md)
- [default_db_path](../../functions/src/config/default_db_path.md)
- [LoggingConfig](../../classes/src/config/LoggingConfig.md)
- [default](../../functions/src/config/LoggingConfig/default/default.md)
- [default_log_level](../../functions/src/config/default_log_level.md)
- [default](../../functions/src/config/Config/default/default.md)
- [load](../../functions/src/config/Config/load.md)
- [load_from_path](../../functions/src/config/Config/load_from_path.md)
- [system_config_path](../../functions/src/config/Config/system_config_path.md)
- [project_config_path](../../functions/src/config/Config/project_config_path.md)
- [local_config_path](../../functions/src/config/Config/local_config_path.md)
- [merge_from_file](../../functions/src/config/Config/merge_from_file.md)
- [merge](../../functions/src/config/Config/merge.md)
- [apply_env_overrides](../../functions/src/config/Config/apply_env_overrides.md)
- [load_provider_api_keys](../../functions/src/config/Config/load_provider_api_keys.md)
- [validate](../../functions/src/config/Config/validate.md)
- [save](../../functions/src/config/Config/save.md)
- [model_override_targets_the_selected_provider](../../functions/src/config/model_override_targets_the_selected_provider.md)
- [model_override_respects_provider_precedence](../../functions/src/config/model_override_respects_provider_precedence.md)
- [model_override_reports_when_no_provider_can_take_it](../../functions/src/config/model_override_reports_when_no_provider_can_take_it.md)
- [model_override_skips_disabled_providers](../../functions/src/config/model_override_skips_disabled_providers.md)
- [model_override_targets_gemini_when_it_is_the_selected_provider](../../functions/src/config/model_override_targets_gemini_when_it_is_the_selected_provider.md)
- [model_override_skips_gemini_without_api_key](../../functions/src/config/model_override_skips_gemini_without_api_key.md)
- [allow_bash_trusts_only_the_listed_read_only_programs](../../functions/src/config/allow_bash_trusts_only_the_listed_read_only_programs.md)
- [empty_security_config_trusts_nothing](../../functions/src/config/empty_security_config_trusts_nothing.md)
- [test_default_config](../../functions/src/config/test_default_config.md)
- [test_config_validation](../../functions/src/config/test_config_validation.md)
- [test_config_validation_invalid_log_level](../../functions/src/config/test_config_validation_invalid_log_level.md)
- [test_config_validation_empty_crabrace_url](../../functions/src/config/test_config_validation_empty_crabrace_url.md)
- [test_config_from_toml](../../functions/src/config/test_config_from_toml.md)
- [test_config_save_and_load](../../functions/src/config/test_config_save_and_load.md)
- [test_config_env_overrides](../../functions/src/config/test_config_env_overrides.md)
- [test_provider_config_api_keys_from_env](../../functions/src/config/test_provider_config_api_keys_from_env.md)
- [test_ollama_config_from_env](../../functions/src/config/test_ollama_config_from_env.md)
- [test_ollama_provider_config_default](../../functions/src/config/test_ollama_provider_config_default.md)
- [test_system_config_path](../../functions/src/config/test_system_config_path.md)
- [test_local_config_path](../../functions/src/config/test_local_config_path.md)
- [test_debug_config_default](../../functions/src/config/test_debug_config_default.md)
- [test_provider_configs_default](../../functions/src/config/test_provider_configs_default.md)
- [test_database_config_default](../../functions/src/config/test_database_config_default.md)
- [test_logging_config_default](../../functions/src/config/test_logging_config_default.md)

# Imports

- `pub use crabrace::{CrabraceConfig, CrabraceIntegration}`
- `pub use secrets::{ProviderSecrets, SecretString}`
- `pub use update::{ProviderUpdater, UpdateResult}`
- `anyhow::{Context, Result}`
- `serde::{Deserialize, Serialize}`
- `std::fs`
- `std::path::{Path, PathBuf}`
- `crate::llm::tools::sandbox::{
            AllowAll, AndPolicy, BashCommandAllowlist, DenyPathPrefixRule, DenyToolRule,
        }`
- `super::*`
- `tempfile::NamedTempFile`
- `crate::llm::tools::sandbox::PolicyDecision`

# Member of

- [crustly](../../packages/crustly.md)