---
type: Rust Function
title: compaction_refuses_when_nothing_would_be_dropped
resource: src/llm/agent/compaction.rs#L468-L477
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/pool_with_compaction_table
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/assistant
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn compaction_refuses_when_nothing_would_be_dropped()`

# Calls

- [pool_with_compaction_table](../../../../../functions/src/llm/agent/compaction/pool_with_compaction_table.md)
- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)
- [assistant](../../../../../functions/src/llm/provider/types/Message/assistant.md)
- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)