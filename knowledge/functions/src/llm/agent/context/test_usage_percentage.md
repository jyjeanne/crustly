---
type: Rust Function
title: test_usage_percentage
resource: src/llm/agent/context.rs#L307-L318
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/usage_percentage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_usage_percentage()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [usage_percentage](../../../../../functions/src/llm/agent/context/AgentContext/usage_percentage.md)