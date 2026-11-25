---
type: Rust Function
title: chat
resource: tests/memory_extraction_test.rs#L166-L173
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
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
  - target: functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn chat(turns: usize) -> Vec<(&'static str, &'static str)>`

# Called by

- [extraction_is_off_by_default_and_calls_nothing](../../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [a_run_saves_what_the_model_proposes_and_moves_the_watermark](../../../functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark.md)
- [below_the_every_n_count_nothing_runs_unless_forced](../../../functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced.md)
- [an_unusable_answer_is_retried_then_skipped](../../../functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [a_session_end_run_writes_an_episode_that_starts_the_next_session](../../../functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [a_candidate_with_an_unknown_scope_is_dropped](../../../functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped.md)
- [the_background_trigger_runs_once_per_session_at_a_time](../../../functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)
- [a_range_the_provider_always_fails_on_is_skipped_after_three_tries](../../../functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries.md)
- [a_long_backlog_is_read_over_several_runs](../../../functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs.md)
- [a_session_end_run_over_a_long_backlog_reads_the_newest_turns](../../../functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns.md)