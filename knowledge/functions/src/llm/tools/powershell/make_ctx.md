---
type: Rust Function
title: make_ctx
resource: src/llm/tools/powershell.rs#L374-L376
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn make_ctx() -> ToolExecutionContext`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_auto_approve](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)

# Called by

- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)