---
type: Rust Function
title: test_would_exceed_limit
resource: src/llm/agent/context.rs#L295-L304
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
  - target: functions/src/llm/agent/context/AgentContext/would_exceed_limit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_would_exceed_limit()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [would_exceed_limit](../../../../../functions/src/llm/agent/context/AgentContext/would_exceed_limit.md)