---
type: Rust Method
title: run_migrations
resource: src/db/mod.rs#L154-L184
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/task/FileLock/acquire
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/reconcile_line_endings
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/database/setup_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/foreign_keys_are_enforced
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/deleting_a_session_cascades_to_its_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/test_file_crud
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/test_file_list_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/test_message_crud
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/test_message_list_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/setup_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/test_session_crud
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/test_session_archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/create_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/forget_memory/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/store_with_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/search_memory/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/create_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/create_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/create_test_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/plan/setup_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/create_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/test_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_test_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/app_with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/test_app
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
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
  - target: functions/tests/error_scenarios_test/create_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/create_test_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/open
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
  - target: functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_consolidate_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/store_and_real_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/database
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_test/migrated_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_transfer_test/store
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/plan_mode_integration_test/setup_test_env
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn run_migrations(&self) -> Result<()>`

# Calls

- [acquire](../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [context](../../../../functions/src/services/ServiceManager/context.md)
- [reconcile_line_endings](../../../../functions/src/db/reconcile_line_endings.md)

# Called by

- [setup_test_db](../../../../functions/benches/database/setup_test_db.md)
- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [cmd_db](../../../../functions/src/cli/cmd_db.md)
- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [foreign_keys_are_enforced](../../../../functions/src/db/foreign_keys_are_enforced.md)
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
- [create_test_db](../../../../functions/tests/error_scenarios_test/create_test_db.md)
- [create_test_db](../../../../functions/tests/integration_test/create_test_db.md)
- [test_database_persistence](../../../../functions/tests/integration_test/test_database_persistence.md)
- [open](../../../../functions/tests/memory_acceptance_test/open.md)
- [setup](../../../../functions/tests/memory_anchors_test/setup.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [setup](../../../../functions/tests/memory_code_knowledge_test/setup.md)
- [without_code_knowledge_nothing_changes](../../../../functions/tests/memory_code_knowledge_test/without_code_knowledge_nothing_changes.md)
- [setup](../../../../functions/tests/memory_consolidate_test/setup.md)
- [database](../../../../functions/tests/memory_extraction_test/database.md)
- [a_database_from_before_memory_upgrades_and_keeps_its_sessions](../../../../functions/tests/memory_migration_test/a_database_from_before_memory_upgrades_and_keeps_its_sessions.md)
- [a_checksum_recorded_with_other_line_endings_is_accepted_and_updated](../../../../functions/tests/memory_migration_test/a_checksum_recorded_with_other_line_endings_is_accepted_and_updated.md)
- [an_unrelated_checksum_mismatch_is_still_refused](../../../../functions/tests/memory_migration_test/an_unrelated_checksum_mismatch_is_still_refused.md)
- [store](../../../../functions/tests/memory_minor_fixes_test/store.md)
- [store_and_project](../../../../functions/tests/memory_paths_test/store_and_project.md)
- [store_and_real_project](../../../../functions/tests/memory_paths_test/store_and_real_project.md)
- [repo_with_projects](../../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [repo](../../../../functions/tests/memory_review_fixes_test/repo.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [database](../../../../functions/tests/memory_semantic_test/database.md)
- [store](../../../../functions/tests/memory_store_test/store.md)
- [migrated_db](../../../../functions/tests/memory_test/migrated_db.md)
- [store](../../../../functions/tests/memory_transfer_test/store.md)
- [setup](../../../../functions/tests/memory_vectors_test/setup.md)
- [crash_recovery_resumes_at_correct_task](../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)
- [setup_test_env](../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)