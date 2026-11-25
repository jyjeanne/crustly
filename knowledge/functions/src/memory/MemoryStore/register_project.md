---
type: Rust Method
title: register_project
resource: src/memory/mod.rs#L219-L221
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/never_returns_another_projects_memories
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/remember_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_preference_survives_a_restart_and_is_recalled_in_another_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_project_fact_is_never_recalled_in_another_repository
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/an_updated_decision_supersedes_the_old_one_and_keeps_its_history
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_anchors_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_code_knowledge_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/setup
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
  - target: functions/tests/memory_extraction_test/the_model_is_never_shown_tool_output_or_secrets
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
  - target: functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/an_approved_memory_is_embedded
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/saving_twice_gives_one_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/replacement_must_stay_in_the_same_scope
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/saves_are_announced_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_includes_own_project_and_global_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_records_what_it_sent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/export_scope_limits_what_is_written
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn register_project(&self, project: &ProjectIdentity) -> Result<()>`

# Calls

- [upsert_project](../../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [memory_block](../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [setup](../../../../functions/src/llm/tools/search_memory/setup.md)
- [never_returns_another_projects_memories](../../../../functions/src/llm/tools/search_memory/never_returns_another_projects_memories.md)
- [import](../../../../functions/src/memory/transfer/import.md)
- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [remember_command](../../../../functions/src/tui/app/App/remember_command.md)
- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [a_preference_survives_a_restart_and_is_recalled_in_another_repository](../../../../functions/tests/memory_acceptance_test/a_preference_survives_a_restart_and_is_recalled_in_another_repository.md)
- [a_project_fact_is_never_recalled_in_another_repository](../../../../functions/tests/memory_acceptance_test/a_project_fact_is_never_recalled_in_another_repository.md)
- [an_updated_decision_supersedes_the_old_one_and_keeps_its_history](../../../../functions/tests/memory_acceptance_test/an_updated_decision_supersedes_the_old_one_and_keeps_its_history.md)
- [a_secret_never_reaches_the_database_file_or_its_wal](../../../../functions/tests/memory_acceptance_test/a_secret_never_reaches_the_database_file_or_its_wal.md)
- [setup](../../../../functions/tests/memory_anchors_test/setup.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [setup](../../../../functions/tests/memory_consolidate_test/setup.md)
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
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
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
- [saving_twice_gives_one_memory](../../../../functions/tests/memory_store_test/saving_twice_gives_one_memory.md)
- [replacement_must_stay_in_the_same_scope](../../../../functions/tests/memory_store_test/replacement_must_stay_in_the_same_scope.md)
- [saves_are_announced_once](../../../../functions/tests/memory_store_test/saves_are_announced_once.md)
- [recall_includes_own_project_and_global_only](../../../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recent_work_is_added_only_on_the_first_request](../../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [recall_records_what_it_sent](../../../../functions/tests/memory_store_test/recall_records_what_it_sent.md)
- [export_then_import_recreates_the_memories_in_a_new_database](../../../../functions/tests/memory_transfer_test/export_then_import_recreates_the_memories_in_a_new_database.md)
- [export_scope_limits_what_is_written](../../../../functions/tests/memory_transfer_test/export_scope_limits_what_is_written.md)