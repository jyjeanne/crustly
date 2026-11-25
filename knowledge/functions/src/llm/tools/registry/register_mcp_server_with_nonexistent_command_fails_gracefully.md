---
type: Rust Function
title: register_mcp_server_with_nonexistent_command_fails_gracefully
resource: src/llm/tools/registry.rs#L493-L514
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register_mcp_server
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/count
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn register_mcp_server_with_nonexistent_command_fails_gracefully()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register_mcp_server](../../../../../functions/src/llm/tools/registry/ToolRegistry/register_mcp_server.md)
- [count](../../../../../functions/src/llm/tools/registry/ToolRegistry/count.md)