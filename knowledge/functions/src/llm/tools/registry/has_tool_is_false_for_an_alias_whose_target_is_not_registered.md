---
type: Rust Function
title: has_tool_is_false_for_an_alias_whose_target_is_not_registered
resource: src/llm/tools/registry.rs#L532-L537
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
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn has_tool_is_false_for_an_alias_whose_target_is_not_registered()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [has_tool](../../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)