---
type: Rust Function
title: build_tool_registry
resource: src/cli/mod.rs#L622-L663
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/registry/ToolRegistry/register
  called_by:
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/cli/build_tool_registry_registers_every_built_in_tool
  - functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers
  - functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server
---

# Signature

`fn build_tool_registry() -> crate::llm::tools::registry::ToolRegistry`

# Calls

- [register](../../../functions/src/llm/tools/registry/ToolRegistry/register.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../functions/src/cli/cmd_run.md)
- [build_tool_registry_registers_every_built_in_tool](../../../functions/src/cli/build_tool_registry_registers_every_built_in_tool.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)