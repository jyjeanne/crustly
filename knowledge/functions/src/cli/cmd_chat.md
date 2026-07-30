---
type: Rust Function
title: cmd_chat
resource: src/cli/mod.rs#L754-L844
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/provider/factory/create_provider
  - functions/src/cli/build_tool_registry
  - functions/src/cli/connect_configured_mcp_servers
  - functions/src/llm/agent/service/AgentService/with_max_tool_iterations
  - functions/src/tui/app/App/set_ollama_host
  - functions/src/cli/ollama_host
  - functions/src/tui/app/App/set_ollama_config
  - functions/src/tui/app/App/set_mcp_status
  - functions/src/tui/app/App/event_sender
  - functions/src/tui/app/App/set_auto_mode_state
  - functions/src/cli/build_approval_callback
  - functions/src/llm/tools/registry/ToolRegistry/set_policy
  - functions/src/config/SecurityConfig/to_policy
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  - functions/src/llm/agent/service/AgentService/with_approval_callback
  - functions/src/tui/app/App/set_agent_service
  called_by:
  - functions/src/cli/run
---

# Signature

`async fn cmd_chat(config: &crate::config::Config, _session_id: Option<String>) -> Result<()>`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_provider](../../../functions/src/llm/provider/factory/create_provider.md)
- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [connect_configured_mcp_servers](../../../functions/src/cli/connect_configured_mcp_servers.md)
- [with_max_tool_iterations](../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [set_ollama_host](../../../functions/src/tui/app/App/set_ollama_host.md)
- [ollama_host](../../../functions/src/cli/ollama_host.md)
- [set_ollama_config](../../../functions/src/tui/app/App/set_ollama_config.md)
- [set_mcp_status](../../../functions/src/tui/app/App/set_mcp_status.md)
- [event_sender](../../../functions/src/tui/app/App/event_sender.md)
- [set_auto_mode_state](../../../functions/src/tui/app/App/set_auto_mode_state.md)
- [build_approval_callback](../../../functions/src/cli/build_approval_callback.md)
- [set_policy](../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [to_policy](../../../functions/src/config/SecurityConfig/to_policy.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_approval_callback](../../../functions/src/llm/agent/service/AgentService/with_approval_callback.md)
- [set_agent_service](../../../functions/src/tui/app/App/set_agent_service.md)

# Called by

- [run](../../../functions/src/cli/run.md)