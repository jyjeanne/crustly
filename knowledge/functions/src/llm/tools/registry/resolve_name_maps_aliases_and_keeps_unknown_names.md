---
type: Rust Function
title: resolve_name_maps_aliases_and_keeps_unknown_names
resource: src/llm/tools/registry.rs#L557-L568
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/registry/ToolRegistry/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/resolve_name
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_name_maps_aliases_and_keeps_unknown_names()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [resolve_name](../../../../../functions/src/llm/tools/registry/ToolRegistry/resolve_name.md)