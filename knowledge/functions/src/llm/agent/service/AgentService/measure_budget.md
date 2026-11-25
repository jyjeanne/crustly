---
type: Rust Method
title: measure_budget
resource: src/llm/agent/service.rs#L933-L947
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/effective_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/from_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/calibration
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn measure_budget(&self, context: &AgentContext, tools: &RequestTools) -> ContextBudget`

# Calls

- [effective_system_prompt](../../../../../../functions/src/llm/agent/context/AgentContext/effective_system_prompt.md)
- [from_counts](../../../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)
- [calibration](../../../../../../functions/src/llm/agent/service/AgentService/calibration.md)

# Called by

- [make_room](../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [prepare_message_context](../../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [make_room_elides_old_tool_output_when_the_budget_is_tight](../../../../../../functions/src/llm/agent/service/make_room_elides_old_tool_output_when_the_budget_is_tight.md)
- [call_model](../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [finish](../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)