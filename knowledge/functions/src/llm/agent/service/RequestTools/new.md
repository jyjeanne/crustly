---
type: Rust Method
title: new
resource: src/llm/agent/service.rs#L75-L78
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/budget/tool_schema_tokens
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn new(defs: Vec<Tool>) -> Self`

# Calls

- [tool_schema_tokens](../../../../../../functions/src/llm/agent/budget/tool_schema_tokens.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)