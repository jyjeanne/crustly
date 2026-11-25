---
type: Rust Method
title: with_auto_approve_tools
resource: src/llm/agent/service.rs#L756-L759
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_auto_approve_tools(mut self, auto_approve: bool) -> Self`

# Called by

- [cmd_run](../../../../../../functions/src/cli/cmd_run.md)
- [test_send_message_with_tool_execution](../../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [stopping_mid_round_answers_every_pending_tool_call](../../../../../../functions/src/llm/agent/service/stopping_mid_round_answers_every_pending_tool_call.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)