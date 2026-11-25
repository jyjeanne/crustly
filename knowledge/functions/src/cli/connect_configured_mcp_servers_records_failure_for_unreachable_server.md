---
type: Rust Function
title: connect_configured_mcp_servers_records_failure_for_unreachable_server
resource: src/cli/mod.rs#L3896-L3912
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn connect_configured_mcp_servers_records_failure_for_unreachable_server()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [connect_configured_mcp_servers](../../../functions/src/cli/connect_configured_mcp_servers.md)