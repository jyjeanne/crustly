---
type: Rust Function
title: make_room_elides_old_tool_output_when_the_budget_is_tight
resource: src/llm/agent/service.rs#L3026-L3058
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/RequestTools/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/measure_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/budget/ContextBudget/is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn make_room_elides_old_tool_output_when_the_budget_is_tight()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [new](../../../../../functions/src/llm/agent/service/RequestTools/new.md)
- [measure_budget](../../../../../functions/src/llm/agent/service/AgentService/measure_budget.md)
- [is_tight](../../../../../functions/src/llm/agent/budget/ContextBudget/is_tight.md)
- [make_room](../../../../../functions/src/llm/agent/service/AgentService/make_room.md)