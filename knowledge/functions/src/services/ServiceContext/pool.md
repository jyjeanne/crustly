---
type: Rust Method
title: pool
resource: src/services/mod.rs#L35-L37
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/memory_test_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/track_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/list_files_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/update_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/delete_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/delete_files_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/file/FileService/count_files_in_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_tool_log
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/delete_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/delete_messages_for_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/count_messages_in_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/test_service_context_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/plan/PlanService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/list_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_title
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/archive_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/unarchive_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/delete_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/get_most_recent_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/count_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/count_archived_sessions
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn pool(&self) -> Pool`

# Called by

- [memory_test_store](../../../../functions/src/llm/agent/service/memory_test_store.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [a_refused_global_save_stores_nothing](../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [track_file](../../../../functions/src/services/file/FileService/track_file.md)
- [get_file](../../../../functions/src/services/file/FileService/get_file.md)
- [list_files_for_session](../../../../functions/src/services/file/FileService/list_files_for_session.md)
- [find_file_by_path](../../../../functions/src/services/file/FileService/find_file_by_path.md)
- [update_file](../../../../functions/src/services/file/FileService/update_file.md)
- [update_file_content](../../../../functions/src/services/file/FileService/update_file_content.md)
- [delete_file](../../../../functions/src/services/file/FileService/delete_file.md)
- [delete_files_for_session](../../../../functions/src/services/file/FileService/delete_files_for_session.md)
- [count_files_in_session](../../../../functions/src/services/file/FileService/count_files_in_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [get_message](../../../../functions/src/services/message/MessageService/get_message.md)
- [list_messages_for_session](../../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [update_message](../../../../functions/src/services/message/MessageService/update_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [delete_message](../../../../functions/src/services/message/MessageService/delete_message.md)
- [delete_messages_for_session](../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [count_messages_in_session](../../../../functions/src/services/message/MessageService/count_messages_in_session.md)
- [test_service_context_creation](../../../../functions/src/services/test_service_context_creation.md)
- [new](../../../../functions/src/services/plan/PlanService/new.md)
- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_session](../../../../functions/src/services/session/SessionService/get_session.md)
- [list_sessions](../../../../functions/src/services/session/SessionService/list_sessions.md)
- [update_session](../../../../functions/src/services/session/SessionService/update_session.md)
- [update_session_title](../../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [archive_session](../../../../functions/src/services/session/SessionService/archive_session.md)
- [unarchive_session](../../../../functions/src/services/session/SessionService/unarchive_session.md)
- [delete_session](../../../../functions/src/services/session/SessionService/delete_session.md)
- [get_most_recent_session](../../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [count_sessions](../../../../functions/src/services/session/SessionService/count_sessions.md)
- [count_archived_sessions](../../../../functions/src/services/session/SessionService/count_archived_sessions.md)