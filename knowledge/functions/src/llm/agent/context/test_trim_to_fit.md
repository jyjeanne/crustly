---
type: Rust Function
title: test_trim_to_fit
resource: src/llm/agent/context.rs#L321-L337
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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
  - target: functions/src/llm/agent/context/AgentContext/trim_to_fit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_trim_to_fit()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [trim_to_fit](../../../../../functions/src/llm/agent/context/AgentContext/trim_to_fit.md)