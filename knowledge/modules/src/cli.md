---
type: Rust Module
title: cli
resource: src/cli/mod.rs#L1-L3923
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-parser-subcommand
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llama-cpp-models-hardwarefit
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llama-cpp-models-checksum-mismatch-error-prefix-disk-space-error-prefix-download-failed-error-prefix-download-start-error-prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-config
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memoryscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-searchscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-self-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memorycategory-memoryscope-memorysource-memorystatus-newmemory-saveoutcome
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-project-identify-transfer-searchscope
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-tools-agent-agenttool-apply-patch-applypatchtool-ask-user-askusertool-bash-bashtool-code-exec-codeexectool-context-contexttool-doc-parser-docparsertool-edit-edittool-glob-globtool-grep-greptool-http-httpclienttool-ls-lstool-notebook-notebookedittool-plan-tool-plantool-powershell-powershelltool-read-readtool-registry-toolregistry-save-memory-savememorytool-skill-skilltool-task-tasktool-todo-write-todowritetool-web-fetch-webfetchtool-web-search-websearchtool-write-writetool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-tui-events-toolapprovalrequest-tuievent
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database-llm-agent-agentservice-services-servicecontext-tui
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-planexecmode
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-plan-planmodestate
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database-llm-agent-agentservice-services-servicecontext-sessionservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-secrets-secretstring
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-ollama-models
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write-as
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llama-cpp-models
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-logging
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-bufread-bufreader
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/clap-commandfactory
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-config-toolprofile
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-budget-contextbudget
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-agent-tool-profile-toolselection-core-tools
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-llama-cpp-models-localggufmodel
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-memory-model-memory-memorycategory-memoryscope-memorysource-memorystatus
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-repository-memory-insertmemory
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [configure_agent](../../functions/src/cli/configure_agent.md)
- [memory_store](../../functions/src/cli/memory_store.md)
- [build_memory_store](../../functions/src/cli/build_memory_store.md)
- [configure_agent_loop](../../functions/src/cli/configure_agent_loop.md)
- [Cli](../../classes/src/cli/Cli.md)
- [Commands](../../classes/src/cli/Commands.md)
- [OllamaCommands](../../classes/src/cli/OllamaCommands.md)
- [LlamaCppCommands](../../classes/src/cli/LlamaCppCommands.md)
- [LlamaCppModelJson](../../classes/src/cli/LlamaCppModelJson.md)
- [from](../../functions/src/cli/LlamaCppModelJson/from-crate-llm-provider-llama-cpp-models-localggufmodel/from.md)
- [LlamaCppListJson](../../classes/src/cli/LlamaCppListJson.md)
- [hardware_fit_label](../../functions/src/cli/hardware_fit_label.md)
- [llama_cpp_exit_code](../../functions/src/cli/llama_cpp_exit_code.md)
- [LogCommands](../../classes/src/cli/LogCommands.md)
- [MemoryCommands](../../classes/src/cli/MemoryCommands.md)
- [DbCommands](../../classes/src/cli/DbCommands.md)
- [KeyringCommands](../../classes/src/cli/KeyringCommands.md)
- [OutputFormat](../../classes/src/cli/OutputFormat.md)
- [run](../../functions/src/cli/run.md)
- [load_config](../../functions/src/cli/load_config.md)
- [cmd_init](../../functions/src/cli/cmd_init.md)
- [cmd_config](../../functions/src/cli/cmd_config.md)
- [display_path](../../functions/src/cli/display_path.md)
- [memory_status_lines](../../functions/src/cli/memory_status_lines.md)
- [memory_line](../../functions/src/cli/memory_line.md)
- [parse_scope](../../functions/src/cli/parse_scope.md)
- [resolve_memory](../../functions/src/cli/resolve_memory.md)
- [confirm](../../functions/src/cli/confirm.md)
- [cmd_memory](../../functions/src/cli/cmd_memory.md)
- [cmd_db](../../functions/src/cli/cmd_db.md)
- [build_tool_registry](../../functions/src/cli/build_tool_registry.md)
- [connect_configured_mcp_servers](../../functions/src/cli/connect_configured_mcp_servers.md)
- [build_approval_callback](../../functions/src/cli/build_approval_callback.md)
- [cmd_chat](../../functions/src/cli/cmd_chat.md)
- [auto_mode_bypasses_approval](../../functions/src/cli/auto_mode_bypasses_approval.md)
- [describe_context_usage](../../functions/src/cli/describe_context_usage.md)
- [cmd_run](../../functions/src/cli/cmd_run.md)
- [cmd_keyring](../../functions/src/cli/cmd_keyring.md)
- [ollama_host](../../functions/src/cli/ollama_host.md)
- [cmd_ollama](../../functions/src/cli/cmd_ollama.md)
- [cmd_ollama](../../functions/src/cli/cmd_ollama-2.md)
- [resolve_llama_cpp_model_path](../../functions/src/cli/resolve_llama_cpp_model_path.md)
- [cmd_llama_cpp](../../functions/src/cli/cmd_llama_cpp.md)
- [DoctorStatus](../../classes/src/cli/DoctorStatus.md)
- [icon](../../functions/src/cli/DoctorStatus/icon.md)
- [DoctorFinding](../../classes/src/cli/DoctorFinding.md)
- [llama_cpp_doctor_findings](../../functions/src/cli/llama_cpp_doctor_findings.md)
- [cmd_llama_cpp](../../functions/src/cli/cmd_llama_cpp-2.md)
- [cmd_autoplan](../../functions/src/cli/cmd_autoplan.md)
- [cmd_logs](../../functions/src/cli/cmd_logs.md)
- [test_cli_parse](../../functions/src/cli/test_cli_parse.md)
- [test_ollama_host_defaults_when_unconfigured](../../functions/src/cli/test_ollama_host_defaults_when_unconfigured.md)
- [test_ollama_host_uses_configured_value](../../functions/src/cli/test_ollama_host_uses_configured_value.md)
- [auto_mode_bypasses_approval_interactive_never_bypasses](../../functions/src/cli/auto_mode_bypasses_approval_interactive_never_bypasses.md)
- [auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only](../../functions/src/cli/auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only.md)
- [autoplan_prompts_for_the_registered_code_and_shell_tool_names](../../functions/src/cli/autoplan_prompts_for_the_registered_code_and_shell_tool_names.md)
- [global_memory_saves_prompt_in_autoplan_but_not_fullauto](../../functions/src/cli/global_memory_saves_prompt_in_autoplan_but_not_fullauto.md)
- [core_profile_fixed_overhead_fits_a_small_window](../../functions/src/cli/core_profile_fixed_overhead_fits_a_small_window.md)
- [autoplan_prompts_for_unlisted_tools_with_risky_capabilities](../../functions/src/cli/autoplan_prompts_for_unlisted_tools_with_risky_capabilities.md)
- [auto_mode_bypasses_approval_fullauto_bypasses_everything](../../functions/src/cli/auto_mode_bypasses_approval_fullauto_bypasses_everything.md)
- [test_ollama_command_parses](../../functions/src/cli/test_ollama_command_parses.md)
- [test_llama_cpp_command_parses](../../functions/src/cli/test_llama_cpp_command_parses.md)
- [test_llama_cpp_list_and_rm_parse](../../functions/src/cli/test_llama_cpp_list_and_rm_parse.md)
- [llama_cpp_list_json_round_trips_with_the_documented_schema](../../functions/src/cli/llama_cpp_list_json_round_trips_with_the_documented_schema.md)
- [llama_cpp_model_json_from_local_gguf_model_carries_every_field](../../functions/src/cli/llama_cpp_model_json_from_local_gguf_model_carries_every_field.md)
- [llama_cpp_exit_code_maps_each_known_message_prefix](../../functions/src/cli/llama_cpp_exit_code_maps_each_known_message_prefix.md)
- [llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context](../../functions/src/cli/llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context.md)
- [llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error](../../functions/src/cli/llama_cpp_exit_code_falls_back_to_one_for_an_unrecognized_error.md)
- [llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok](../../functions/src/cli/llama_cpp_doctor_findings_reports_a_writable_models_dir_as_ok.md)
- [llama_cpp_doctor_findings_warns_on_a_missing_models_dir](../../functions/src/cli/llama_cpp_doctor_findings_warns_on_a_missing_models_dir.md)
- [llama_cpp_doctor_findings_reports_extra_model_paths_existence](../../functions/src/cli/llama_cpp_doctor_findings_reports_extra_model_paths_existence.md)
- [llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in](../../functions/src/cli/llama_cpp_doctor_findings_says_nothing_about_ollama_when_not_opted_in.md)
- [llama_cpp_doctor_findings_reports_build_features](../../functions/src/cli/llama_cpp_doctor_findings_reports_build_features.md)
- [llama_cpp_doctor_findings_always_reports_detected_hardware](../../functions/src/cli/llama_cpp_doctor_findings_always_reports_detected_hardware.md)
- [llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present](../../functions/src/cli/llama_cpp_doctor_findings_skips_the_best_fit_finding_when_no_models_present.md)
- [hardware_fit_label_matches_the_documented_wording](../../functions/src/cli/hardware_fit_label_matches_the_documented_wording.md)
- [resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir](../../functions/src/cli/resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir.md)
- [build_tool_registry_registers_every_built_in_tool](../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)
- [scope_flags_accept_three_values_and_refuse_others](../../functions/src/cli/scope_flags_accept_three_values_and_refuse_others.md)
- [display_path_drops_the_windows_verbatim_prefix](../../functions/src/cli/display_path_drops_the_windows_verbatim_prefix.md)
- [memory_lines_show_a_short_id_scope_category_and_text](../../functions/src/cli/memory_lines_show_a_short_id_scope_category_and_text.md)
- [reject_refuses_a_memory_that_is_not_pending](../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [memory_status_reports_semantic_recall_and_extraction_when_on](../../functions/src/cli/memory_status_reports_semantic_recall_and_extraction_when_on.md)

# Imports

- `anyhow::{Context, Result}`
- `clap::{Parser, Subcommand}`
- `std::sync::Arc`
- `crate::llm::provider::llama_cpp_models::HardwareFit`
- `crate::llm::provider::llama_cpp_models::{
    CHECKSUM_MISMATCH_ERROR_PREFIX, DISK_SPACE_ERROR_PREFIX, DOWNLOAD_FAILED_ERROR_PREFIX,
    DOWNLOAD_START_ERROR_PREFIX,
}`
- `crate::config::Config`
- `crate::memory::model::{MemoryScope, MemorySource, MemoryStatus}`
- `crate::memory::model::MemoryScope`
- `crate::memory::SearchScope`
- `std::io::{self, Write}`
- `crate::memory::model::{
        MemoryCategory, MemoryScope, MemorySource, MemoryStatus, NewMemory, SaveOutcome,
    }`
- `crate::memory::{project::identify, transfer, SearchScope}`
- `crate::db::Database`
- `crate::llm::tools::{
        agent::AgentTool, apply_patch::ApplyPatchTool, ask_user::AskUserTool, bash::BashTool,
        code_exec::CodeExecTool, context::ContextTool, doc_parser::DocParserTool, edit::EditTool,
        glob::GlobTool, grep::GrepTool, http::HttpClientTool, ls::LsTool,
        notebook::NotebookEditTool, plan_tool::PlanTool, powershell::PowerShellTool,
        read::ReadTool, registry::ToolRegistry, save_memory::SaveMemoryTool, skill::SkillTool,
        task::TaskTool, todo_write::TodoWriteTool, web_fetch::WebFetchTool,
        web_search::WebSearchTool, write::WriteTool,
    }`
- `crate::tui::events::{ToolApprovalRequest, TuiEvent}`
- `tokio::sync::mpsc`
- `crate::{db::Database, llm::agent::AgentService, services::ServiceContext, tui}`
- `crate::config::PlanExecMode`
- `crate::plan::PlanModeState`
- `crate::{
        db::Database,
        llm::agent::AgentService,
        services::{ServiceContext, SessionService},
    }`
- `crate::config::secrets::SecretString`
- `crate::llm::provider::ollama_models`
- `std::io::Write as _`
- `crate::llm::provider::llama_cpp_models`
- `crate::logging`
- `std::io::{BufRead, BufReader}`
- `super::*`
- `clap::CommandFactory`
- `crate::config::ToolProfile`
- `crate::llm::agent::budget::ContextBudget`
- `crate::llm::agent::tool_profile::{ToolSelection, CORE_TOOLS}`
- `crate::llm::provider::llama_cpp_models::LocalGgufModel`
- `crate::memory::model::{
            Memory, MemoryCategory, MemoryScope, MemorySource, MemoryStatus,
        }`
- `crate::db::repository::memory::InsertMemory`

# Member of

- [crustly](../../packages/crustly.md)