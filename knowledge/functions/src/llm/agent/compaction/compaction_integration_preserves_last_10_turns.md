---
type: Rust Function
title: compaction_integration_preserves_last_10_turns
resource: src/llm/agent/compaction.rs#L557-L620
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
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/is_plain_user_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn compaction_integration_preserves_last_10_turns()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)
- [is_plain_user_message](../../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)