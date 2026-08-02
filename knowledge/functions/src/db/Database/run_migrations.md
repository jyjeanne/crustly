---
type: Rust Method
title: run_migrations
resource: src/db/mod.rs#L148-L174
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/tools/task/FileLock/acquire
  called_by:
  - functions/benches/database/setup_test_db
  - functions/src/cli/cmd_db
  - functions/src/cli/cmd_chat
  - functions/src/cli/cmd_run
  - functions/src/db/foreign_keys_are_enforced
  - functions/src/db/deleting_a_session_cascades_to_its_messages
  - functions/src/db/migrating_from_pre_modernization_schema_preserves_existing_messages
  - functions/src/db/repository/file/test_file_crud
  - functions/src/db/repository/file/test_file_list_by_session
  - functions/src/db/repository/message/test_message_crud
  - functions/src/db/repository/message/test_message_list_by_session
  - functions/src/db/repository/plan/setup_test_db
  - functions/src/db/repository/session/test_session_crud
  - functions/src/db/repository/session/test_session_archive
  - functions/src/llm/agent/service/create_test_service
  - functions/src/llm/agent/service/sub_agent_launcher_does_not_auto_approve_tools
  - functions/src/llm/agent/service/test_send_message_with_tool_execution
  - functions/src/services/file/create_test_service
  - functions/src/services/message/create_test_service
  - functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
  - functions/src/services/create_test_pool
  - functions/src/services/plan/setup_test_service
  - functions/src/services/session/create_test_service
  - functions/src/tui/app/test_app
  - functions/src/tui/render/test_app
  - functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set
  - functions/tests/compaction_test/compaction_preserves_last_10_turns
  - functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns
  - functions/tests/compaction_test/compaction_writes_one_record_to_db
  - functions/tests/error_scenarios_test/create_test_db
  - functions/tests/integration_test/create_test_db
  - functions/tests/integration_test/test_database_persistence
  - functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task
  - functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order
  - functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp
  - functions/tests/plan_mode_integration_test/setup_test_env
---

# Signature

`pub async fn run_migrations(&self) -> Result<()>`

# Calls

- [acquire](../../../../functions/src/llm/tools/task/FileLock/acquire.md)

# Called by

- [setup_test_db](../../../../functions/benches/database/setup_test_db.md)
- [cmd_db](../../../../functions/src/cli/cmd_db.md)
- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_run](../../../../functions/src/cli/cmd_run.md)
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
- [sub_agent_launcher_does_not_auto_approve_tools](../../../../functions/src/llm/agent/service/sub_agent_launcher_does_not_auto_approve_tools.md)
- [test_send_message_with_tool_execution](../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [create_test_service](../../../../functions/src/services/file/create_test_service.md)
- [create_test_service](../../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [create_test_pool](../../../../functions/src/services/create_test_pool.md)
- [setup_test_service](../../../../functions/src/services/plan/setup_test_service.md)
- [create_test_service](../../../../functions/src/services/session/create_test_service.md)
- [test_app](../../../../functions/src/tui/app/test_app.md)
- [test_app](../../../../functions/src/tui/render/test_app.md)
- [run_loop_exits_immediately_when_should_quit_is_set](../../../../functions/src/tui/runner/run_loop_exits_immediately_when_should_quit_is_set.md)
- [compaction_preserves_last_10_turns](../../../../functions/tests/compaction_test/compaction_preserves_last_10_turns.md)
- [compaction_fails_gracefully_with_insufficient_turns](../../../../functions/tests/compaction_test/compaction_fails_gracefully_with_insufficient_turns.md)
- [compaction_writes_one_record_to_db](../../../../functions/tests/compaction_test/compaction_writes_one_record_to_db.md)
- [create_test_db](../../../../functions/tests/error_scenarios_test/create_test_db.md)
- [create_test_db](../../../../functions/tests/integration_test/create_test_db.md)
- [test_database_persistence](../../../../functions/tests/integration_test/test_database_persistence.md)
- [crash_recovery_resumes_at_correct_task](../../../../functions/tests/plan_crash_recovery_test/crash_recovery_resumes_at_correct_task.md)
- [task_state_transitions_correct_order](../../../../functions/tests/plan_crash_recovery_test/task_state_transitions_correct_order.md)
- [failed_task_stores_error_without_completion_timestamp](../../../../functions/tests/plan_crash_recovery_test/failed_task_stores_error_without_completion_timestamp.md)
- [setup_test_env](../../../../functions/tests/plan_mode_integration_test/setup_test_env.md)