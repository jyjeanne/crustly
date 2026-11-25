---
type: Rust Function
title: plan_compaction
resource: src/llm/agent/compaction.rs#L184-L200
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/message_has_tool_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/is_plain_user_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn plan_compaction(messages: &[Message], keep: usize) -> Option<CompactionPlan>`

# Calls

- [message_has_tool_result](../../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [is_plain_user_message](../../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)

# Called by

- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)