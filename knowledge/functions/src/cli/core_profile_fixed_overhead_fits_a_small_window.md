---
type: Rust Function
title: core_profile_fixed_overhead_fits_a_small_window
resource: src/cli/mod.rs#L3232-L3269
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
  - target: functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/tool_profile/ToolSelection/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/measure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/fixed_overhead
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn core_profile_fixed_overhead_fits_a_small_window()`

# Calls

- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [get_tool_definitions](../../../functions/src/llm/tools/registry/ToolRegistry/get_tool_definitions.md)
- [resolve](../../../functions/src/llm/agent/tool_profile/ToolSelection/resolve.md)
- [exposes](../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [measure](../../../functions/src/llm/agent/budget/ContextBudget/measure.md)
- [fixed_overhead](../../../functions/src/llm/agent/budget/ContextBudget/fixed_overhead.md)
- [is_tight](../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)