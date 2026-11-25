---
type: Rust Method
title: default
resource: src/config/mod.rs#L1363-L1387
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/crabrace/CrabraceConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/default_db_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/default_log_level
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/ToolCacheConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/AgentConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/VerificationConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/SubAgentsConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/FallbackConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/test_ollama_host_defaults_when_unconfigured
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/test_ollama_host_uses_configured_value
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_reports_build_features
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/from_layers
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_default_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_validation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_validation_invalid_log_level
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_validation_empty_crabrace_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_save_and_load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_from_layers_without_files_is_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_config_env_overrides
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_provider_config_api_keys_from_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/test_ollama_config_from_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/factory/named_providers_must_be_known_and_configured
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_config_loading
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default](../../../../../functions/src/config/crabrace/CrabraceConfig/default/default.md)
- [default_db_path](../../../../../functions/src/config/default_db_path.md)
- [default_log_level](../../../../../functions/src/config/default_log_level.md)
- [default](../../../../../functions/src/config/MemoryConfig/default/default.md)
- [default](../../../../../functions/src/config/ToolCacheConfig/default/default.md)
- [default](../../../../../functions/src/config/AgentConfig/default/default.md)
- [default](../../../../../functions/src/config/VerificationConfig/default/default.md)
- [default](../../../../../functions/src/config/SubAgentsConfig/default/default.md)
- [default](../../../../../functions/src/config/FallbackConfig/default/default.md)

# Called by

- [cmd_init](../../../../../functions/src/cli/cmd_init.md)
- [test_ollama_host_defaults_when_unconfigured](../../../../../functions/src/cli/test_ollama_host_defaults_when_unconfigured.md)
- [test_ollama_host_uses_configured_value](../../../../../functions/src/cli/test_ollama_host_uses_configured_value.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [llama_cpp_doctor_findings_warns_on_a_missing_models_dir](../../../../../functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir.md)
- [llama_cpp_doctor_findings_reports_extra_model_paths_existence](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence.md)
- [llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in](../../../../../functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in.md)
- [llama_cpp_doctor_findings_reports_build_features](../../../../../functions/src/cli/llama_cpp_doctor_findings_reports_build_features.md)
- [llama_cpp_doctor_findings_always_reports_detected_hardware](../../../../../functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware.md)
- [llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present](../../../../../functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../../../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../../../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [memory_status_reports_semantic_recall_and_extraction_when_on](../../../../../functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on.md)
- [from_layers](../../../../../functions/src/config/Config/from_layers.md)
- [test_default_config](../../../../../functions/src/config/test_default_config.md)
- [test_config_validation](../../../../../functions/src/config/test_config_validation.md)
- [test_config_validation_invalid_log_level](../../../../../functions/src/config/test_config_validation_invalid_log_level.md)
- [test_config_validation_empty_crabrace_url](../../../../../functions/src/config/test_config_validation_empty_crabrace_url.md)
- [test_config_save_and_load](../../../../../functions/src/config/test_config_save_and_load.md)
- [test_from_layers_without_files_is_default](../../../../../functions/src/config/test_from_layers_without_files_is_default.md)
- [test_config_env_overrides](../../../../../functions/src/config/test_config_env_overrides.md)
- [test_provider_config_api_keys_from_env](../../../../../functions/src/config/test_provider_config_api_keys_from_env.md)
- [test_ollama_config_from_env](../../../../../functions/src/config/test_ollama_config_from_env.md)
- [named_providers_must_be_known_and_configured](../../../../../functions/src/llm/provider/factory/named_providers_must_be_known_and_configured.md)
- [test_config_loading](../../../../../functions/tests/integration_test/test_config_loading.md)