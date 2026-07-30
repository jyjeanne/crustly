---
type: Rust Function
title: cmd_run
resource: src/cli/mod.rs#L875-L990
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/llm/provider/factory/create_provider
  - functions/src/cli/build_tool_registry
  - functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
  - functions/src/llm/tools/registry/ToolRegistry/set_policy
  - functions/src/config/SecurityConfig/to_policy
  - functions/src/llm/agent/service/AgentService/with_tool_registry
  - functions/src/llm/agent/service/AgentService/with_max_tool_iterations
  - functions/src/llm/agent/service/AgentService/with_auto_approve_tools
  - functions/src/llm/agent/service/AgentService/send_message_with_tools
  called_by:
  - functions/src/cli/run
  - functions/src/cli/cmd_autoplan
---

# Signature

`async fn cmd_run( config: &crate::config::Config, prompt: String, auto_approve: bool, format: OutputFormat, ) -> Result<()>`

# Calls

- [run_migrations](../../../functions/src/db/Database/run_migrations.md)
- [create_provider](../../../functions/src/llm/provider/factory/create_provider.md)
- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [register_mcp_server](../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [set_policy](../../../functions/src/llm/tools/registry/ToolRegistry/set_policy.md)
- [to_policy](../../../functions/src/config/SecurityConfig/to_policy.md)
- [with_tool_registry](../../../functions/src/llm/agent/service/AgentService/with_tool_registry.md)
- [with_max_tool_iterations](../../../functions/src/llm/agent/service/AgentService/with_max_tool_iterations.md)
- [with_auto_approve_tools](../../../functions/src/llm/agent/service/AgentService/with_auto_approve_tools.md)
- [send_message_with_tools](../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)

# Called by

- [run](../../../functions/src/cli/run.md)
- [cmd_autoplan](../../../functions/src/cli/cmd_autoplan.md)