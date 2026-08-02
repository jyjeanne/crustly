---
type: Rust Module
title: cli
resource: src/cli/mod.rs#L1-L1795
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/clap-parser-subcommand
  - external/std-sync-arc
  - external/crate-config-config
  - external/crate-db-database
  - external/std-io-self-write
  - external/crate-llm-tools-agent-agenttool-apply-patch-applypatchtool-ask-user-askusertool-bash-bashtool-code-exec-codeexectool-context-contexttool-doc-parser-docparsertool-edit-edittool-glob-globtool-grep-greptool-http-httpclienttool-ls-lstool-notebook-notebookedittool-plan-tool-plantool-powershell-powershelltool-read-readtool-registry-toolregistry-save-memory-savememorytool-skill-skilltool-task-tasktool-todo-write-todowritetool-web-fetch-webfetchtool-web-search-websearchtool-write-writetool
  - external/crate-tui-events-toolapprovalrequest-tuievent
  - external/tokio-sync-mpsc
  - external/crate-db-database-llm-agent-agentservice-services-servicecontext-tui
  - external/crate-config-planexecmode
  - external/crate-plan-planmodestate
  - external/crate-db-database-llm-agent-agentservice-services-servicecontext-sessionservice
  - external/crate-config-secrets-secretstring
  - external/crate-llm-provider-ollama-models
  - external/std-io-write-as
  - external/crate-llm-provider-llama-cpp-models
  - external/crate-logging
  - external/std-io-bufread-bufreader
  - external/super
  - external/clap-commandfactory
  member_of:
  - packages/crustly
---

# Contains

- [Cli](../../classes/src/cli/Cli.md)
- [Commands](../../classes/src/cli/Commands.md)
- [OllamaCommands](../../classes/src/cli/OllamaCommands.md)
- [LlamaCppCommands](../../classes/src/cli/LlamaCppCommands.md)
- [LogCommands](../../classes/src/cli/LogCommands.md)
- [DbCommands](../../classes/src/cli/DbCommands.md)
- [KeyringCommands](../../classes/src/cli/KeyringCommands.md)
- [OutputFormat](../../classes/src/cli/OutputFormat.md)
- [run](../../functions/src/cli/run.md)
- [load_config](../../functions/src/cli/load_config.md)
- [cmd_init](../../functions/src/cli/cmd_init.md)
- [cmd_config](../../functions/src/cli/cmd_config.md)
- [cmd_db](../../functions/src/cli/cmd_db.md)
- [build_tool_registry](../../functions/src/cli/build_tool_registry.md)
- [connect_configured_mcp_servers](../../functions/src/cli/connect_configured_mcp_servers.md)
- [build_approval_callback](../../functions/src/cli/build_approval_callback.md)
- [cmd_chat](../../functions/src/cli/cmd_chat.md)
- [auto_mode_bypasses_approval](../../functions/src/cli/auto_mode_bypasses_approval.md)
- [cmd_run](../../functions/src/cli/cmd_run.md)
- [cmd_keyring](../../functions/src/cli/cmd_keyring.md)
- [ollama_host](../../functions/src/cli/ollama_host.md)
- [cmd_ollama](../../functions/src/cli/cmd_ollama.md)
- [cmd_ollama](../../functions/src/cli/cmd_ollama-2.md)
- [resolve_llama_cpp_model_path](../../functions/src/cli/resolve_llama_cpp_model_path.md)
- [cmd_llama_cpp](../../functions/src/cli/cmd_llama_cpp.md)
- [cmd_llama_cpp](../../functions/src/cli/cmd_llama_cpp-2.md)
- [cmd_autoplan](../../functions/src/cli/cmd_autoplan.md)
- [cmd_logs](../../functions/src/cli/cmd_logs.md)
- [test_cli_parse](../../functions/src/cli/test_cli_parse.md)
- [test_ollama_host_defaults_when_unconfigured](../../functions/src/cli/test_ollama_host_defaults_when_unconfigured.md)
- [test_ollama_host_uses_configured_value](../../functions/src/cli/test_ollama_host_uses_configured_value.md)
- [auto_mode_bypasses_approval_interactive_never_bypasses](../../functions/src/cli/auto_mode_bypasses_approval_interactive_never_bypasses.md)
- [auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only](../../functions/src/cli/auto_mode_bypasses_approval_autoplan_gates_high_risk_tools_only.md)
- [known_gap_powershell_is_not_classified_as_high_risk](../../functions/src/cli/known_gap_powershell_is_not_classified_as_high_risk.md)
- [auto_mode_bypasses_approval_fullauto_bypasses_everything](../../functions/src/cli/auto_mode_bypasses_approval_fullauto_bypasses_everything.md)
- [test_ollama_command_parses](../../functions/src/cli/test_ollama_command_parses.md)
- [test_llama_cpp_command_parses](../../functions/src/cli/test_llama_cpp_command_parses.md)
- [test_llama_cpp_list_and_rm_parse](../../functions/src/cli/test_llama_cpp_list_and_rm_parse.md)
- [resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir](../../functions/src/cli/resolve_llama_cpp_model_path_treats_bare_names_as_relative_to_models_dir.md)
- [build_tool_registry_registers_every_built_in_tool](../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)

# Imports

- `anyhow::{Context, Result}`
- `clap::{Parser, Subcommand}`
- `std::sync::Arc`
- `crate::config::Config`
- `crate::db::Database`
- `std::io::{self, Write}`
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

# Member of

- [crustly](../../packages/crustly.md)