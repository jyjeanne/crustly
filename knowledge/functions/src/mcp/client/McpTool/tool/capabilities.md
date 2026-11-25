---
type: Rust Method
title: capabilities
resource: src/mcp/client.rs#L336-L338
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/mcp/client/mcp_tool_always_requires_approval_regardless_of_empty_capabilities
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn capabilities(&self) -> Vec<ToolCapability>`

# Called by

- [mcp_tool_always_requires_approval_regardless_of_empty_capabilities](../../../../../../functions/src/mcp/client/mcp_tool_always_requires_approval_regardless_of_empty_capabilities.md)