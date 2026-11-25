---
type: Rust Function
title: mcp_tool_always_requires_approval_regardless_of_empty_capabilities
resource: src/mcp/client.rs#L522-L562
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/mcp/client/McpTool/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/mcp/client/McpTool/tool/capabilities
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn mcp_tool_always_requires_approval_regardless_of_empty_capabilities()`

# Calls

- [new](../../../../functions/src/mcp/client/McpTool/new.md)
- [capabilities](../../../../functions/src/mcp/client/McpTool/tool/capabilities.md)