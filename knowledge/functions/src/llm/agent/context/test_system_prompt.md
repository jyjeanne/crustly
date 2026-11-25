---
type: Rust Function
title: test_system_prompt
resource: src/llm/agent/context.rs#L278-L285
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
  - target: functions/src/llm/agent/context/AgentContext/with_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_system_prompt()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [with_system_prompt](../../../../../functions/src/llm/agent/context/AgentContext/with_system_prompt.md)