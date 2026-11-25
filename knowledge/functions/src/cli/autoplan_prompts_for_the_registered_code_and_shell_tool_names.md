---
type: Rust Function
title: autoplan_prompts_for_the_registered_code_and_shell_tool_names
resource: src/cli/mod.rs#L3361-L3375
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/cli/build_tool_registry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/auto_mode_bypasses_approval
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn autoplan_prompts_for_the_registered_code_and_shell_tool_names()`

# Calls

- [build_tool_registry](../../../functions/src/cli/build_tool_registry.md)
- [has_tool](../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)
- [auto_mode_bypasses_approval](../../../functions/src/cli/auto_mode_bypasses_approval.md)