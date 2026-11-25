---
type: Rust Function
title: compaction_fires_at_threshold
resource: src/llm/agent/compaction.rs#L480-L514
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
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/should_compact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/usage_percentage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn compaction_fires_at_threshold()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [should_compact](../../../../../functions/src/llm/agent/context/AgentContext/should_compact.md)
- [usage_percentage](../../../../../functions/src/llm/agent/context/AgentContext/usage_percentage.md)