---
type: Rust Function
title: compact
resource: src/llm/agent/compaction.rs#L46-L48
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/compaction/compact_keeping
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
    resolved_by: tree-sitter
    confidence: exact
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

`pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord>`

# Calls

- [compact_keeping](../../../../../functions/src/llm/agent/compaction/compact_keeping.md)

# Called by

- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)
- [compaction_preserves_last_10_turns](../../../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)