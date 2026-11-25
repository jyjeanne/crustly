---
type: Rust Method
title: new
resource: src/llm/agent/service/subagents.rs#L131-L137
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_subagents
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(max_concurrent: usize) -> Self`

# Called by

- [new](../../../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_subagents](../../../../../../../functions/src/llm/agent/service/AgentService/with_subagents.md)