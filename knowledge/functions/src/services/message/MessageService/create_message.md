---
type: Rust Method
title: create_message
resource: src/services/message.rs#L24-L61
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/create
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round
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
  - target: functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn create_message( &self, session_id: Uuid, role: String, content: String, ) -> Result<Message>`

# Calls

- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [create](../../../../../functions/src/db/repository/message/MessageRepository/create.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [context_overflow_compacts_and_retries_the_round](../../../../../functions/src/llm/agent/service/context_overflow_compacts_and_retries_the_round.md)
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
- [clear_session_is_refused_while_the_current_session_is_processing](../../../../../functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing.md)
- [clear_session_proceeds_when_only_another_session_is_processing](../../../../../functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing.md)