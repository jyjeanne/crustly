---
type: Rust Function
title: make_ctx
resource: src/llm/tools/powershell.rs#L374-L376
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve
  called_by:
  - functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode
  - functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode
---

# Signature

`fn make_ctx() -> ToolExecutionContext`

# Calls

- [with_auto_approve](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_auto_approve.md)

# Called by

- [execute_blocks_dangerous_command_in_read_only_mode](../../../../../functions/src/llm/tools/powershell/execute_blocks_dangerous_command_in_read_only_mode.md)
- [execute_allows_read_only_command_in_plan_mode](../../../../../functions/src/llm/tools/powershell/execute_allows_read_only_command_in_plan_mode.md)