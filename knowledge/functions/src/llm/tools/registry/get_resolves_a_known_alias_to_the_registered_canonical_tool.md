---
type: Rust Function
title: get_resolves_a_known_alias_to_the_registered_canonical_tool
resource: src/llm/tools/registry.rs#L519-L529
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
  - target: functions/src/llm/tools/registry/ToolRegistry/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn get_resolves_a_known_alias_to_the_registered_canonical_tool()`

# Calls

- [new](../../../../../functions/src/llm/tools/registry/ToolRegistry/new.md)
- [register](../../../../../functions/src/llm/tools/registry/ToolRegistry/register.md)
- [get](../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [has_tool](../../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)