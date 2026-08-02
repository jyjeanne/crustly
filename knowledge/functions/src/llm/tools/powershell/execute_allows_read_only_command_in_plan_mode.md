---
type: Rust Function
title: execute_allows_read_only_command_in_plan_mode
resource: src/llm/tools/powershell.rs#L498-L511
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/powershell/make_ctx
  - functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode
---

# Signature

`async fn execute_allows_read_only_command_in_plan_mode()`

# Calls

- [make_ctx](../../../../../functions/src/llm/tools/powershell/make_ctx.md)
- [with_read_only_mode](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_read_only_mode.md)