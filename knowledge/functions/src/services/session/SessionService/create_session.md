---
type: Rust Method
title: create_session
resource: src/services/session.rs#L27-L48
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/create_test_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/test_send_message_with_tool_execution
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_track_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_get_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_list_files_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_update_file_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_delete_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_delete_files_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_count_files_in_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_is_file_tracked
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_get_or_create_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/test_get_files_with_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_get_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_metrics_with_perf_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_metrics_without_perf_data
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_delete_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_delete_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_count_messages_in_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_get_last_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_get_messages_by_role
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_calculate_totals
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_get_session_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_update_session_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_update_session_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_archive_unarchive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_delete_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_list_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_get_most_recent_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_count_sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_api_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_rate_limit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_timeout
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_invalid_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_authentication
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_empty_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_database_concurrent_access
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/error_scenarios_test/test_error_recovery_after_failure
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_simple_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_multi_turn_conversation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_session_management
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_cost_tracking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_token_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_end_to_end_system_prompt
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/integration_test/test_database_persistence
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn create_session(&self, title: Option<String>) -> Result<Session>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [create](../../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [cmd_run](../../../../../functions/src/cli/cmd_run.md)
- [create_test_service](../../../../../functions/src/llm/agent/service/create_test_service.md)
- [test_send_message_with_tool_execution](../../../../../functions/src/llm/agent/service/test_send_message_with_tool_execution.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [execute](../../../../../functions/src/llm/agent/service/subagents/execute.md)
- [test_track_file](../../../../../functions/src/services/file/test_track_file.md)
- [test_get_file](../../../../../functions/src/services/file/test_get_file.md)
- [test_list_files_for_session](../../../../../functions/src/services/file/test_list_files_for_session.md)
- [test_find_file_by_path](../../../../../functions/src/services/file/test_find_file_by_path.md)
- [test_update_file_content](../../../../../functions/src/services/file/test_update_file_content.md)
- [test_delete_file](../../../../../functions/src/services/file/test_delete_file.md)
- [test_delete_files_for_session](../../../../../functions/src/services/file/test_delete_files_for_session.md)
- [test_count_files_in_session](../../../../../functions/src/services/file/test_count_files_in_session.md)
- [test_is_file_tracked](../../../../../functions/src/services/file/test_is_file_tracked.md)
- [test_get_or_create_file](../../../../../functions/src/services/file/test_get_or_create_file.md)
- [test_get_files_with_content](../../../../../functions/src/services/file/test_get_files_with_content.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [test_create_message](../../../../../functions/src/services/message/test_create_message.md)
- [test_get_message](../../../../../functions/src/services/message/test_get_message.md)
- [test_list_messages_for_session](../../../../../functions/src/services/message/test_list_messages_for_session.md)
- [test_update_message_usage](../../../../../functions/src/services/message/test_update_message_usage.md)
- [test_update_message_metrics_with_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)
- [test_delete_message](../../../../../functions/src/services/message/test_delete_message.md)
- [test_delete_messages_for_session](../../../../../functions/src/services/message/test_delete_messages_for_session.md)
- [test_count_messages_in_session](../../../../../functions/src/services/message/test_count_messages_in_session.md)
- [test_get_last_message](../../../../../functions/src/services/message/test_get_last_message.md)
- [test_get_messages_by_role](../../../../../functions/src/services/message/test_get_messages_by_role.md)
- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)
- [test_create_session](../../../../../functions/src/services/session/test_create_session.md)
- [test_get_session](../../../../../functions/src/services/session/test_get_session.md)
- [test_get_session_required](../../../../../functions/src/services/session/test_get_session_required.md)
- [test_update_session_title](../../../../../functions/src/services/session/test_update_session_title.md)
- [test_update_session_usage](../../../../../functions/src/services/session/test_update_session_usage.md)
- [test_archive_unarchive_session](../../../../../functions/src/services/session/test_archive_unarchive_session.md)
- [test_delete_session](../../../../../functions/src/services/session/test_delete_session.md)
- [test_list_sessions](../../../../../functions/src/services/session/test_list_sessions.md)
- [test_get_most_recent_session](../../../../../functions/src/services/session/test_get_most_recent_session.md)
- [test_count_sessions](../../../../../functions/src/services/session/test_count_sessions.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [test_error_api_error](../../../../../functions/tests/error_scenarios_test/test_error_api_error.md)
- [test_error_rate_limit](../../../../../functions/tests/error_scenarios_test/test_error_rate_limit.md)
- [test_error_timeout](../../../../../functions/tests/error_scenarios_test/test_error_timeout.md)
- [test_error_invalid_response](../../../../../functions/tests/error_scenarios_test/test_error_invalid_response.md)
- [test_error_authentication](../../../../../functions/tests/error_scenarios_test/test_error_authentication.md)
- [test_error_empty_message](../../../../../functions/tests/error_scenarios_test/test_error_empty_message.md)
- [test_error_database_concurrent_access](../../../../../functions/tests/error_scenarios_test/test_error_database_concurrent_access.md)
- [test_error_recovery_after_failure](../../../../../functions/tests/error_scenarios_test/test_error_recovery_after_failure.md)
- [test_end_to_end_simple_message](../../../../../functions/tests/integration_test/test_end_to_end_simple_message.md)
- [test_end_to_end_multi_turn_conversation](../../../../../functions/tests/integration_test/test_end_to_end_multi_turn_conversation.md)
- [test_end_to_end_session_management](../../../../../functions/tests/integration_test/test_end_to_end_session_management.md)
- [test_end_to_end_cost_tracking](../../../../../functions/tests/integration_test/test_end_to_end_cost_tracking.md)
- [test_end_to_end_token_usage](../../../../../functions/tests/integration_test/test_end_to_end_token_usage.md)
- [test_end_to_end_system_prompt](../../../../../functions/tests/integration_test/test_end_to_end_system_prompt.md)
- [test_database_persistence](../../../../../functions/tests/integration_test/test_database_persistence.md)