---
type: Rust Function
title: compact
resource: src/llm/agent/compaction.rs#L27-L124
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/agent/compaction/message_has_tool_result
  - functions/src/llm/agent/compaction/summarise_turns
  - functions/src/llm/agent/context/token_count
  called_by:
  - functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged
  - functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns
  - functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/logging/init_minimal_logging
  - functions/tests/compaction_test/compaction_preserves_last_10_turns
  - functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns
  - functions/tests/compaction_test/compaction_writes_one_record_to_db
---

# Signature

`pub async fn compact(ctx: &mut AgentContext, pool: &sqlx::SqlitePool) -> Result<CompactionRecord>`

# Calls

- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [message_has_tool_result](../../../../../functions/src/llm/agent/compaction/message_has_tool_result.md)
- [summarise_turns](../../../../../functions/src/llm/agent/compaction/summarise_turns.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)

# Called by

- [compaction_atomicity_db_failure_leaves_context_unchanged](../../../../../functions/src/llm/agent/compaction/compaction_atomicity_db_failure_leaves_context_unchanged.md)
- [compaction_integration_preserves_last_10_turns](../../../../../functions/src/llm/agent/compaction/compaction_integration_preserves_last_10_turns.md)
- [compaction_never_splits_a_tool_use_result_pair](../../../../../functions/src/llm/agent/compaction/compaction_never_splits_a_tool_use_result_pair.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [init_minimal_logging](../../../../../functions/src/logging/init_minimal_logging.md)
- [compaction_preserves_last_10_turns](../../../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)