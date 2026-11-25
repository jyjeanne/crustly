---
type: Rust Function
title: build_context
resource: tests/compaction_test.rs#L39-L53
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
  - target: functions/tests/compaction_test/text_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/compaction_test/compaction_preserves_last_10_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/compaction_test/compaction_writes_one_record_to_db
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn build_context(session_id: Uuid, n: usize, max_tokens: usize) -> AgentContext`

# Calls

- [new](../../../functions/src/llm/agent/context/AgentContext/new.md)
- [add_message](../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [text_message](../../../functions/tests/compaction_test/text_message.md)

# Called by

- [compaction_preserves_last_10_turns](../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)