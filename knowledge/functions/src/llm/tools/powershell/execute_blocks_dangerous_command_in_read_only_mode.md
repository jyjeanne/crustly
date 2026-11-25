---
type: Rust Function
title: execute_blocks_dangerous_command_in_read_only_mode
resource: src/llm/tools/powershell.rs#L482-L495
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute_blocks_dangerous_command_in_read_only_mode()`

# Calls

- [make_ctx](../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [with_read_only_mode](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)
- [execute](../../../../../functions/src/llm/tools/powershell/PowerShellTool/tool/execute.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)