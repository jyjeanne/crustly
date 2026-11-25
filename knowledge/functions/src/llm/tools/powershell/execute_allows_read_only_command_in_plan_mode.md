---
type: Rust Function
title: execute_allows_read_only_command_in_plan_mode
resource: src/llm/tools/powershell.rs#L498-L511
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/powershell/make_ctx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/powershell/PowerShellTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute_allows_read_only_command_in_plan_mode()`

# Calls

- [make_ctx](../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [with_read_only_mode](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [execute](../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)