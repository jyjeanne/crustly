---
type: Rust Method
title: extract
resource: src/memory/extraction.rs#L120-L269
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/extraction_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/turns_after
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/should_extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extract/transcript
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/search
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_temperature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/record_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/set_extraction_state
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/trigger
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/finish_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn extract( &self, session: Uuid, project: &ProjectIdentity, session_end: bool, force: bool, ) -> Result<usize>`

# Calls

- [extraction_state](../../../../../functions/src/db/repository/memory/MemoryRepository/extraction_state.md)
- [turns_after](../../../../../functions/src/db/repository/memory/MemoryRepository/turns_after.md)
- [should_extract](../../../../../functions/src/memory/extract/should_extract.md)
- [upsert_project](../../../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [transcript](../../../../../functions/src/memory/extract/transcript.md)
- [search](../../../../../functions/src/memory/MemoryStore/search.md)
- [new](../../../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_temperature](../../../../../functions/src/llm/provider/types/LLMRequest/with_temperature.md)
- [record_failure](../../../../../functions/src/memory/extraction/MemoryStore/record_failure.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)
- [save_with](../../../../../functions/src/memory/MemoryStore/save_with.md)
- [set_extraction_state](../../../../../functions/src/db/repository/memory/MemoryRepository/set_extraction_state.md)
- [user](../../../../../functions/src/llm/provider/types/Message/user.md)

# Called by

- [cmd_run](../../../../../functions/src/cli/cmd_run.md)
- [trigger](../../../../../functions/src/memory/extraction/MemoryStore/trigger.md)
- [finish_session](../../../../../functions/src/memory/extraction/MemoryStore/finish_session.md)
- [the_model_is_never_shown_tool_output_or_secrets](../../../../../functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../../../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../../../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)
- [a_long_backlog_is_read_over_several_runs](../../../../../functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs.md)
- [a_session_end_run_over_a_long_backlog_reads_the_newest_turns](../../../../../functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns.md)
- [extraction_is_off_by_default_and_calls_nothing](../../../../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [a_run_saves_what_the_model_proposes_and_moves_the_watermark](../../../../../functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark.md)
- [below_the_every_n_count_nothing_runs_unless_forced](../../../../../functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced.md)
- [an_unusable_answer_is_retried_then_skipped](../../../../../functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped.md)
- [a_session_end_run_writes_an_episode_that_starts_the_next_session](../../../../../functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session.md)
- [a_candidate_with_an_unknown_scope_is_dropped](../../../../../functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped.md)
- [a_range_the_provider_always_fails_on_is_skipped_after_three_tries](../../../../../functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries.md)