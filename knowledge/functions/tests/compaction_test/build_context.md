---
type: Rust Function
title: build_context
resource: tests/compaction_test.rs#L39-L53
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/context/AgentContext/add_message
  - functions/tests/compaction_test/text_message
  called_by:
  - functions/tests/compaction_test/compaction_preserves_last_10_turns
  - functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns
  - functions/tests/compaction_test/compaction_writes_one_record_to_db
---

# Signature

`fn build_context(session_id: Uuid, n: usize, max_tokens: usize) -> AgentContext`

# Calls

- [add_message](../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [text_message](../../../functions/tests/compaction_test/text_message.md)

# Called by

- [compaction_preserves_last_10_turns](../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)