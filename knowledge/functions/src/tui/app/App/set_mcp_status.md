---
type: Rust Method
title: set_mcp_status
resource: src/tui/app.rs#L986-L988
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_mcp_status(&mut self, status: Vec<crate::mcp::McpServerStatus>)`

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)