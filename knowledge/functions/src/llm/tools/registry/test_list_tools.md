---
type: Rust Function
title: test_list_tools
resource: src/llm/tools/registry.rs#L397-L413
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
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/list_tools
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_list_tools()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [list_tools](../../../../../functions/src/llm/tools/registry/ToolRegistry/list_tools.md)