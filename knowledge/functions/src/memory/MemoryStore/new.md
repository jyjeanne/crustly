---
type: Rust Method
title: new
resource: src/memory/mod.rs#L104-L119
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/sanitized
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/build_memory_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/memory_test_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/store_with_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_test_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_anchors_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/lessons_need_extraction_on
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_minor_fixes_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_store_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_transfer_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(pool: SqlitePool, config: MemoryConfig) -> Self`

# Calls

- [new](../../../../functions/src/db/repository/memory/MemoryRepository/new.md)
- [sanitized](../../../../functions/src/config/MemoryConfig/sanitized.md)

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [build_memory_store](../../../../functions/src/cli/build_memory_store.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [memory_test_store](../../../../functions/src/llm/agent/service/memory_test_store.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [setup](../../../../functions/src/llm/tools/forget_memory/setup.md)
- [store_with_db](../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [setup](../../../../functions/src/llm/tools/search_memory/setup.md)
- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [app_with_extraction](../../../../functions/src/tui/app/app_with_extraction.md)
- [open](../../../../functions/tests/memory_acceptance_test/open.md)
- [setup](../../../../functions/tests/memory_anchors_test/setup.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [without_code_knowledge_nothing_changes](../../../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)
- [setup](../../../../functions/tests/memory_consolidate_test/setup.md)
- [three_episodes_become_one_lesson_with_its_sources](../../../../functions/tests/memory_consolidate_test/three_episodes_become_one_lesson_with_its_sources.md)
- [episodes_feed_one_lesson_only](../../../../functions/tests/memory_consolidate_test/episodes_feed_one_lesson_only.md)
- [episodes_archived_by_the_same_run_do_not_feed_a_lesson](../../../../functions/tests/memory_consolidate_test/episodes_archived_by_the_same_run_do_not_feed_a_lesson.md)
- [lessons_need_extraction_on](../../../../functions/tests/memory_consolidate_test/lessons_need_extraction_on.md)
- [extraction_is_off_by_default_and_calls_nothing](../../../../functions/tests/memory_extraction_test/extraction_is_off_by_default_and_calls_nothing.md)
- [a_run_saves_what_the_model_proposes_and_moves_the_watermark](../../../../functions/tests/memory_extraction_test/a_run_saves_what_the_model_proposes_and_moves_the_watermark.md)
- [below_the_every_n_count_nothing_runs_unless_forced](../../../../functions/tests/memory_extraction_test/below_the_every_n_count_nothing_runs_unless_forced.md)
- [the_model_is_never_shown_tool_output_or_secrets](../../../../functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets.md)
- [an_unusable_answer_is_retried_then_skipped](../../../../functions/tests/memory_extraction_test/an_unusable_answer_is_retried_then_skipped.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [a_session_end_run_writes_an_episode_that_starts_the_next_session](../../../../functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [a_candidate_with_an_unknown_scope_is_dropped](../../../../functions/tests/memory_extraction_test/a_candidate_with_an_unknown_scope_is_dropped.md)
- [the_background_trigger_runs_once_per_session_at_a_time](../../../../functions/tests/memory_extraction_test/the_background_trigger_runs_once_per_session_at_a_time.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)
- [a_range_the_provider_always_fails_on_is_skipped_after_three_tries](../../../../functions/tests/memory_extraction_test/a_range_the_provider_always_fails_on_is_skipped_after_three_tries.md)
- [a_long_backlog_is_read_over_several_runs](../../../../functions/tests/memory_extraction_test/a_long_backlog_is_read_over_several_runs.md)
- [a_session_end_run_over_a_long_backlog_reads_the_newest_turns](../../../../functions/tests/memory_extraction_test/a_session_end_run_over_a_long_backlog_reads_the_newest_turns.md)
- [an_episode_in_recent_work_is_not_listed_twice](../../../../functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)
- [store](../../../../functions/tests/memory_minor_fixes_test/store.md)
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [when_the_embedder_is_down_keyword_recall_still_works](../../../../functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works.md)
- [a_failed_embedding_is_retried_and_then_stored](../../../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)
- [store](../../../../functions/tests/memory_store_test/store.md)
- [store](../../../../functions/tests/memory_transfer_test/store.md)