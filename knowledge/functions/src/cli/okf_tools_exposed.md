---
type: Rust Function
title: okf_tools_exposed
resource: src/cli/mod.rs#L1890-L1896
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent_loop
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/connect_configured_mcp_servers
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn okf_tools_exposed(config: &crate::config::Config) -> bool`

# Called by

- [configure_agent_loop](../../../functions/src/cli/configure_agent_loop.md)
- [connect_configured_mcp_servers](../../../functions/src/cli/connect_configured_mcp_servers.md)