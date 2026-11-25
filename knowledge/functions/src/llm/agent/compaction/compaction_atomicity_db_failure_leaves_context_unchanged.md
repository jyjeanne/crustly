---
type: Rust Function
title: compaction_atomicity_db_failure_leaves_context_unchanged
resource: src/llm/agent/compaction.rs#L517-L554
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
---

# Signature

`async fn compaction_atomicity_db_failure_leaves_context_unchanged()`

# Calls

- [new](../../../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [compact](../../../../../functions/src/llm/agent/compaction/compact.md)