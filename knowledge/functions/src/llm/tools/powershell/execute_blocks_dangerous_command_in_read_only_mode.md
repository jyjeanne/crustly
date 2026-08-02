---
type: Rust Function
title: execute_blocks_dangerous_command_in_read_only_mode
resource: src/llm/tools/powershell.rs#L482-L495
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/powershell/make_ctx
  - functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
---

# Signature

`async fn execute_blocks_dangerous_command_in_read_only_mode()`

# Calls

- [make_ctx](../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [with_read_only_mode](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)