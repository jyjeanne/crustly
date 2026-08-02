---
type: Rust Method
title: register_mcp_server
resource: src/llm/tools/registry.rs#L226-L253
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/mcp/client/MCPClient/discover_tools
  called_by:
  - functions/src/cli/connect_configured_mcp_servers
  - functions/src/cli/cmd_run
  - functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully
---

# Signature

`pub async fn register_mcp_server( &mut self, server_name: &str, command: &str, args: &[&str], ) -> anyhow::Result<usize>`

# Calls

- [discover_tools](../../../../../../functions/src/mcp/client/MCPClient/discover_tools.md)

# Called by

- [connect_configured_mcp_servers](../../../../../../functions/src/cli/connect_configured_mcp_servers.md)
- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [register_mcp_server_with_nonexistent_command_fails_gracefully](../../../../../../functions/src/llm/tools/registry/register_mcp_server_with_nonexistent_command_fails_gracefully.md)