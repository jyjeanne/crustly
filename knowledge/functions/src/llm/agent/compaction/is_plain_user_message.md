---
type: Rust Function
title: is_plain_user_message
resource: src/llm/agent/compaction.rs#L208-L210
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/message_has_tool_result
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/plan_compaction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn is_plain_user_message(msg: &Message) -> bool`

# Calls

- [message_has_tool_result](../../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)

# Called by

- [plan_compaction](../../../../../functions/src/llm/agent/compaction/plan_compaction.md)
- [compaction_of_one_long_request_keeps_the_task_and_latest_rounds](../../../../../functions/src/llm/agent/compaction/compaction_of_one_long_request_keeps_the_task_and_latest_rounds.md)
- [compaction_integration_preserves_last_10_turns](../../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)