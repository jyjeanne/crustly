---
type: Rust Function
title: cmd_chat
resource: src/cli/mod.rs#L2021-L2129
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/memory_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/set_ollama_host
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/ollama_host
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_ollama_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_llama_cpp_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_llama_cpp_discovery_sources
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_llama_cpp_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_mcp_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/event_sender
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_auto_mode_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/build_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/set_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/SecurityConfig/to_policy
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/with_approval_callback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_agent_service
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/run
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn cmd_chat(config: &crate::config::Config, _session_id: Option<String>) -> Result<()>`

# Calls

- [connect](../../../functions/src/db/Database/connect.md)
- [context](../../../functions/src/services/ServiceManager/context.md)
- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_provider](../../../functions/src/llm/provider/factory/create_provider.md)
- [memory_store](../../../functions/src/cli/memory_store.md)
- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [connect_configured_mcp_servers](../../../functions/src/cli/connect_configured_mcp_servers.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [configure_agent_loop](../../../functions/src/cli/configure_agent_loop.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_memory](../../../functions/src/llm/agent/service/AgentService/with_memory.md)
- [with_system_prompt](../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [with_working_directory](../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [set_ollama_host](../../../functions/src/tui/app/App/set_ollama_host.md)
- [ollama_host](../../../functions/src/cli/ollama_host.md)
- [set_memory](../../../functions/src/tui/app/App/set_memory.md)
- [set_ollama_config](../../../functions/src/tui/app/App/set_ollama_config.md)
- [set_llama_cpp_models_dir](../../../functions/src/tui/app/App/set_llama_cpp_models_dir.md)
- [llama_cpp_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_models_dir.md)
- [set_llama_cpp_discovery_sources](../../../functions/src/tui/app/App/set_llama_cpp_discovery_sources.md)
- [llama_cpp_extra_model_paths](../../../functions/src/config/ProviderConfigs/llama_cpp_extra_model_paths.md)
- [llama_cpp_ollama_models_dir](../../../functions/src/config/ProviderConfigs/llama_cpp_ollama_models_dir.md)
- [set_llama_cpp_config](../../../functions/src/tui/app/App/set_llama_cpp_config.md)
- [set_mcp_status](../../../functions/src/tui/app/App/set_mcp_status.md)
- [event_sender](../../../functions/src/tui/app/App/event_sender.md)
- [set_auto_mode_state](../../../functions/src/tui/app/App/set_auto_mode_state.md)
- [build_approval_callback](../../../functions/src/cli/build_approval_callback.md)
- [set_policy](../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [to_policy](../../../functions/src/config/SecurityConfig/to_policy.md)
- [configure_agent](../../../functions/src/cli/configure_agent.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_approval_callback](../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [set_agent_service](../../../functions/src/tui/app/App/set_agent_service.md)

# Called by

- [run](../../../functions/src/cli/run.md)