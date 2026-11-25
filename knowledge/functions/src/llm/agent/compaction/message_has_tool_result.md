---
type: Rust Function
title: message_has_tool_result
resource: src/llm/agent/compaction.rs#L202-L206
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/plan_compaction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/is_plain_user_message
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn message_has_tool_result(msg: &Message) -> bool`

# Called by

- [elide_tool_results](../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [plan_compaction](../../../../../functions/src/llm/agent/compaction/plan_compaction.md)
- [is_plain_user_message](../../../../../functions/src/llm/agent/compaction/is_plain_user_message.md)