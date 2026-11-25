---
type: Rust Function
title: calibration_learns_from_the_provider_and_resets_on_switch
resource: src/llm/agent/service.rs#L3077-L3099
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/budget/ContextBudget/from_counts
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/record_calibration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/set_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/calibration
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn calibration_learns_from_the_provider_and_resets_on_switch()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [from_counts](../../../../../functions/src/llm/agent/budget/ContextBudget/from_counts.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [record_calibration](../../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)
- [set_provider](../../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [calibration](../../../../../functions/src/llm/agent/service/AgentService/calibration.md)