---
type: Rust Function
title: connect_configured_mcp_servers
resource: src/cli/mod.rs#L1739-L1777
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn connect_configured_mcp_servers( tool_registry: &mut crate::llm::tools::registry::ToolRegistry, config: &crate::config::Config, ) -> Vec<crate::mcp::McpServerStatus>`

# Calls

- [register_mcp_server](../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)

# Called by

- [cmd_chat](../../../functions/src/cli/cmd_chat.md)
- [connect_configured_mcp_servers_returns_empty_status_with_no_servers](../../../functions/src/cli/connect_configured_mcp_servers_returns_empty_status_with_no_servers.md)
- [connect_configured_mcp_servers_records_failure_for_unreachable_server](../../../functions/src/cli/connect_configured_mcp_servers_records_failure_for_unreachable_server.md)