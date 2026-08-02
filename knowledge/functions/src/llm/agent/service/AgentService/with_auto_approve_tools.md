---
type: Rust Method
title: with_auto_approve_tools
resource: src/llm/agent/service.rs#L585-L588
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/cli/cmd_run
  - functions/src/llm/agent/service/test_send_message_with_tool_execution
---

# Signature

`pub fn with_auto_approve_tools(mut self, auto_approve: bool) -> Self`

# Called by

- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)