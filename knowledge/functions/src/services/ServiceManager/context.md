---
type: Rust Method
title: context
resource: src/services/mod.rs#L84-L86
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_init
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_ollama
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/cmd_logs
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/CrabraceIntegration/fetch_providers
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/CrabraceIntegration/health_check
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/save
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/find_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/find_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/find_by_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/create
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/count_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/file/FileRepository/delete_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/upsert_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/set_pending_replaces
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/promote
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/put_embedding
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/find_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/find_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/create
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/count_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/get_last_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/delete_by_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/find_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/find_by_session_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/create
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/plan_from_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/task_from_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/plan_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanRepository/task_to_db
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/create_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/update_task_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/list_tasks_for_plan
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/find_by_id
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/create
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/delete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/list_active
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/list_archived
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/archive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/unarchive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/update_stats
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/retry/retry_db_anyhow
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/try_create_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/factory/create_anthropic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/ollama_models/list_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/mcp/client/MCPClient/connect
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/track_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/get_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/find_file_by_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/update_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/update_file_content
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/delete_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/file/FileService/delete_files_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/create_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/get_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_tool_log
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/update_message_metrics
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/delete_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/delete_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/test_service_manager_creation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session_title
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/archive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/unarchive_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/delete_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn context(&self) -> &ServiceContext`

# Called by

- [cmd_init](../../../../functions/src/cli/cmd_init.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [cmd_chat](../../../../functions/src/cli/cmd_chat.md)
- [cmd_ollama](../../../../functions/src/cli/cmd_ollama.md)
- [cmd_llama_cpp](../../../../functions/src/cli/cmd_llama_cpp.md)
- [cmd_logs](../../../../functions/src/cli/cmd_logs.md)
- [llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context](../../../../functions/src/cli/llama_cpp_exit_code_checks_the_whole_chain_not_just_the_outermost_context.md)
- [fetch_providers](../../../../functions/src/config/crabrace/CrabraceIntegration/fetch_providers.md)
- [health_check](../../../../functions/src/config/crabrace/CrabraceIntegration/health_check.md)
- [save](../../../../functions/src/config/Config/save.md)
- [connect](../../../../functions/src/db/Database/connect.md)
- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [find_by_id](../../../../functions/src/db/repository/file/FileRepository/find_by_id.md)
- [find_by_session](../../../../functions/src/db/repository/file/FileRepository/find_by_session.md)
- [find_by_path](../../../../functions/src/db/repository/file/FileRepository/find_by_path.md)
- [create](../../../../functions/src/db/repository/file/FileRepository/create.md)
- [update](../../../../functions/src/db/repository/file/FileRepository/update.md)
- [delete](../../../../functions/src/db/repository/file/FileRepository/delete.md)
- [count_by_session](../../../../functions/src/db/repository/file/FileRepository/count_by_session.md)
- [delete_by_session](../../../../functions/src/db/repository/file/FileRepository/delete_by_session.md)
- [upsert_project](../../../../functions/src/db/repository/memory/MemoryRepository/upsert_project.md)
- [insert](../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [set_pending_replaces](../../../../functions/src/db/repository/memory/MemoryRepository/set_pending_replaces.md)
- [promote](../../../../functions/src/db/repository/memory/MemoryRepository/promote.md)
- [delete](../../../../functions/src/db/repository/memory/MemoryRepository/delete.md)
- [put_embedding](../../../../functions/src/db/repository/memory/MemoryRepository/put_embedding.md)
- [find_by_id](../../../../functions/src/db/repository/message/MessageRepository/find_by_id.md)
- [find_by_session](../../../../functions/src/db/repository/message/MessageRepository/find_by_session.md)
- [create](../../../../functions/src/db/repository/message/MessageRepository/create.md)
- [update](../../../../functions/src/db/repository/message/MessageRepository/update.md)
- [delete](../../../../functions/src/db/repository/message/MessageRepository/delete.md)
- [count_by_session](../../../../functions/src/db/repository/message/MessageRepository/count_by_session.md)
- [get_last_message](../../../../functions/src/db/repository/message/MessageRepository/get_last_message.md)
- [delete_by_session](../../../../functions/src/db/repository/message/MessageRepository/delete_by_session.md)
- [find_by_id](../../../../functions/src/db/repository/plan/PlanRepository/find_by_id.md)
- [find_by_session_id](../../../../functions/src/db/repository/plan/PlanRepository/find_by_session_id.md)
- [find_tasks_by_plan_id](../../../../functions/src/db/repository/plan/PlanRepository/find_tasks_by_plan_id.md)
- [create](../../../../functions/src/db/repository/plan/PlanRepository/create.md)
- [update](../../../../functions/src/db/repository/plan/PlanRepository/update.md)
- [delete](../../../../functions/src/db/repository/plan/PlanRepository/delete.md)
- [plan_from_db](../../../../functions/src/db/repository/plan/PlanRepository/plan_from_db.md)
- [task_from_db](../../../../functions/src/db/repository/plan/PlanRepository/task_from_db.md)
- [plan_to_db](../../../../functions/src/db/repository/plan/PlanRepository/plan_to_db.md)
- [task_to_db](../../../../functions/src/db/repository/plan/PlanRepository/task_to_db.md)
- [create_task](../../../../functions/src/db/repository/plan/PlanTaskRepository/create_task.md)
- [update_task_status](../../../../functions/src/db/repository/plan/PlanTaskRepository/update_task_status.md)
- [get_task](../../../../functions/src/db/repository/plan/PlanTaskRepository/get_task.md)
- [list_tasks_for_plan](../../../../functions/src/db/repository/plan/PlanTaskRepository/list_tasks_for_plan.md)
- [get_incomplete_tasks](../../../../functions/src/db/repository/plan/PlanTaskRepository/get_incomplete_tasks.md)
- [find_by_id](../../../../functions/src/db/repository/session/SessionRepository/find_by_id.md)
- [create](../../../../functions/src/db/repository/session/SessionRepository/create.md)
- [update](../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [delete](../../../../functions/src/db/repository/session/SessionRepository/delete.md)
- [list](../../../../functions/src/db/repository/session/SessionRepository/list.md)
- [list_active](../../../../functions/src/db/repository/session/SessionRepository/list_active.md)
- [list_archived](../../../../functions/src/db/repository/session/SessionRepository/list_archived.md)
- [archive](../../../../functions/src/db/repository/session/SessionRepository/archive.md)
- [unarchive](../../../../functions/src/db/repository/session/SessionRepository/unarchive.md)
- [update_stats](../../../../functions/src/db/repository/session/SessionRepository/update_stats.md)
- [count](../../../../functions/src/db/repository/session/SessionRepository/count.md)
- [retry_db_anyhow](../../../../functions/src/db/retry/retry_db_anyhow.md)
- [try_create_llama_cpp](../../../../functions/src/llm/provider/factory/try_create_llama_cpp.md)
- [create_anthropic](../../../../functions/src/llm/provider/factory/create_anthropic.md)
- [download_model](../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [list_models](../../../../functions/src/llm/provider/ollama_models/list_models.md)
- [connect](../../../../functions/src/mcp/client/MCPClient/connect.md)
- [extract](../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [track_file](../../../../functions/src/services/file/FileService/track_file.md)
- [get_file](../../../../functions/src/services/file/FileService/get_file.md)
- [find_file_by_path](../../../../functions/src/services/file/FileService/find_file_by_path.md)
- [update_file](../../../../functions/src/services/file/FileService/update_file.md)
- [update_file_content](../../../../functions/src/services/file/FileService/update_file_content.md)
- [delete_file](../../../../functions/src/services/file/FileService/delete_file.md)
- [delete_files_for_session](../../../../functions/src/services/file/FileService/delete_files_for_session.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [get_message](../../../../functions/src/services/message/MessageService/get_message.md)
- [update_message](../../../../functions/src/services/message/MessageService/update_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [delete_message](../../../../functions/src/services/message/MessageService/delete_message.md)
- [delete_messages_for_session](../../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [test_service_manager_creation](../../../../functions/src/services/test_service_manager_creation.md)
- [create_session](../../../../functions/src/services/session/SessionService/create_session.md)
- [get_session](../../../../functions/src/services/session/SessionService/get_session.md)
- [update_session](../../../../functions/src/services/session/SessionService/update_session.md)
- [update_session_title](../../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [archive_session](../../../../functions/src/services/session/SessionService/archive_session.md)
- [unarchive_session](../../../../functions/src/services/session/SessionService/unarchive_session.md)
- [delete_session](../../../../functions/src/services/session/SessionService/delete_session.md)