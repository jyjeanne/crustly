---
type: Rust Method
title: pool
resource: src/db/mod.rs#L121-L123
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/foreign_keys_are_enforced
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/bundled_sqlite_supports_fts5_secure_delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/deleting_a_session_cascades_to_its_messages
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/plan/setup_test_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/test_context
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
  - target: functions/src/llm/tools/save_memory/db_context
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/store_with_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/create_test_pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/create_test_service
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/test_app
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
  - target: functions/src/tui/render/test_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/compaction_preserves_last_10_turns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/compaction_test/compaction_writes_one_record_to_db
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_empty_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/error_scenarios_test/test_error_database_concurrent_access
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/insert_at
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_consolidate_test/merge_band_vectors
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/session_with
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
  - target: functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused
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
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_review_fixes_test/repo
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
  - target: functions/tests/memory_semantic_test/wait_for_vector
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
  - target: functions/tests/memory_test/long_term_memory_tables_exist
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/global_memory_must_not_have_a_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/project_memory_must_have_a_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/fts_index_follows_insert_update_and_delete
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/global_facts_are_unique_across_the_global_scope
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_test/secure_delete_is_on_for_every_connection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_transfer_test/store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_vectors_test/setup
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn pool(&self) -> &SqlitePool`

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [cmd_db](../../../../functions/src/cli/cmd_db.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [foreign_keys_are_enforced](../../../../functions/src/db/foreign_keys_are_enforced.md)
- [bundled_sqlite_supports_fts5_secure_delete](../../../../functions/src/db/bundled_sqlite_supports_fts5_secure_delete.md)
- [deleting_a_session_cascades_to_its_messages](../../../../functions/src/db/deleting_a_session_cascades_to_its_messages.md)
- [migrating_from_pre_modernization_schema_preserves_existing_messages](../../../../functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages.md)
- [test_file_crud](../../../../functions/src/db/repository/file/test_file_crud.md)
- [test_file_list_by_session](../../../../functions/src/db/repository/file/test_file_list_by_session.md)
- [test_message_crud](../../../../functions/src/db/repository/message/test_message_crud.md)
- [test_message_list_by_session](../../../../functions/src/db/repository/message/test_message_list_by_session.md)
- [setup_test_db](../../../../functions/src/db/repository/plan/setup_test_db.md)
- [test_session_crud](../../../../functions/src/db/repository/session/test_session_crud.md)
- [test_session_archive](../../../../functions/src/db/repository/session/test_session_archive.md)
- [create_test_service](../../../../functions/src/llm/agent/service/create_test_service.md)
- [test_send_message_with_tool_execution](../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [test_context](../../../../functions/src/llm/agent/service/test_context.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [setup](../../../../functions/src/llm/tools/forget_memory/setup.md)
- [db_context](../../../../functions/src/llm/tools/save_memory/db_context.md)
- [store_with_db](../../../../functions/src/llm/tools/save_memory/store_with_db.md)
- [setup](../../../../functions/src/llm/tools/search_memory/setup.md)
- [create_test_service](../../../../functions/src/services/file/create_test_service.md)
- [create_test_service](../../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [create_test_pool](../../../../functions/src/services/create_test_pool.md)
- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [create_test_service](../../../../functions/src/services/session/create_test_service.md)
- [test_app](../../../../functions/src/tui/app/test_app.md)
- [memory_test_app](../../../../functions/src/tui/app/memory_test_app.md)
- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [app_with_extraction](../../../../functions/src/tui/app/app_with_extraction.md)
- [test_app](../../../../functions/src/tui/render/test_app.md)
- [run_loop_exits_immediately_when_should_quit_is_set](../../../../functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set.md)
- [compaction_preserves_last_10_turns](../../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)
- [create_error_agent](../../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [test_error_empty_message](../../../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_error_database_concurrent_access](../../../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [create_test_agent](../../../../functions/tests/integration_test/create_test_agent.md)
- [test_end_to_end_system_prompt](../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
- [test_database_persistence](../../../../functions/tests/integration_test/test_database_persistence.md)
- [open](../../../../functions/tests/memory_acceptance_test/open.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../functions/tests/memory_consolidate_test/setup.md)
- [insert_at](../../../../functions/tests/memory_consolidate_test/insert_at.md)
- [merge_band_vectors](../../../../functions/tests/memory_consolidate_test/merge_band_vectors.md)
- [session_with](../../../../functions/tests/memory_extraction_test/session_with.md)
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
- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_unrelated_checksum_mismatch_is_still_refused](../../../../functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused.md)
- [store](../../../../functions/tests/memory_minor_fixes_test/store.md)
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [repo_with_projects](../../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [repo](../../../../functions/tests/memory_review_fixes_test/repo.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [a_memory_is_recalled_by_meaning_without_a_shared_word](../../../../functions/tests/memory_semantic_test/a_memory_is_recalled_by_meaning_without_a_shared_word.md)
- [an_unrelated_memory_below_the_threshold_is_not_recalled](../../../../functions/tests/memory_semantic_test/an_unrelated_memory_below_the_threshold_is_not_recalled.md)
- [when_the_embedder_is_down_keyword_recall_still_works](../../../../functions/tests/memory_semantic_test/when_the_embedder_is_down_keyword_recall_still_works.md)
- [a_failed_embedding_is_retried_and_then_stored](../../../../functions/tests/memory_semantic_test/a_failed_embedding_is_retried_and_then_stored.md)
- [reindex_embeds_memories_saved_before_the_embedder_was_configured](../../../../functions/tests/memory_semantic_test/reindex_embeds_memories_saved_before_the_embedder_was_configured.md)
- [two_memories_saying_the_same_thing_are_recalled_once](../../../../functions/tests/memory_semantic_test/two_memories_saying_the_same_thing_are_recalled_once.md)
- [one_of_two_near_duplicates_is_archived_not_deleted](../../../../functions/tests/memory_semantic_test/one_of_two_near_duplicates_is_archived_not_deleted.md)
- [wait_for_vector](../../../../functions/tests/memory_semantic_test/wait_for_vector.md)
- [an_edited_memory_is_embedded_again_from_its_new_text](../../../../functions/tests/memory_semantic_test/an_edited_memory_is_embedded_again_from_its_new_text.md)
- [an_approved_memory_is_embedded](../../../../functions/tests/memory_semantic_test/an_approved_memory_is_embedded.md)
- [a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget](../../../../functions/tests/memory_semantic_test/a_clear_semantic_match_beats_a_weak_keyword_match_under_a_tight_budget.md)
- [store](../../../../functions/tests/memory_store_test/store.md)
- [long_term_memory_tables_exist](../../../../functions/tests/memory_test/long_term_memory_tables_exist.md)
- [global_memory_must_not_have_a_project](../../../../functions/tests/memory_test/global_memory_must_not_have_a_project.md)
- [project_memory_must_have_a_project](../../../../functions/tests/memory_test/project_memory_must_have_a_project.md)
- [fts_index_follows_insert_update_and_delete](../../../../functions/tests/memory_test/fts_index_follows_insert_update_and_delete.md)
- [second_active_copy_of_a_fact_is_rejected](../../../../functions/tests/memory_test/second_active_copy_of_a_fact_is_rejected.md)
- [superseded_copy_of_a_fact_is_allowed](../../../../functions/tests/memory_test/superseded_copy_of_a_fact_is_allowed.md)
- [global_facts_are_unique_across_the_global_scope](../../../../functions/tests/memory_test/global_facts_are_unique_across_the_global_scope.md)
- [secure_delete_is_on_for_every_connection](../../../../functions/tests/memory_test/secure_delete_is_on_for_every_connection.md)
- [store](../../../../functions/tests/memory_transfer_test/store.md)
- [setup](../../../../functions/tests/memory_vectors_test/setup.md)
- [crash_recovery_resumes_at_correct_task](../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)
- [setup_test_env](../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)