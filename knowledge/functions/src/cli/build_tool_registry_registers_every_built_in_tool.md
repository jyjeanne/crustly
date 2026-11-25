---
type: Rust Function
title: build_tool_registry_registers_every_built_in_tool
resource: src/cli/mod.rs#L3678-L3710
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/count
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_tool_registry_registers_every_built_in_tool()`

# Calls

- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [count](../../../functions/src/llm/tools/registry/ToolRegistry/count.md)
- [has_tool](../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)