---
type: Rust Method
title: create_message
resource: src/services/message.rs#L24-L60
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
  - functions/src/services/message/test_create_message
  - functions/src/services/message/test_get_message
  - functions/src/services/message/test_list_messages_for_session
  - functions/src/services/message/test_update_message_usage
  - functions/src/services/message/test_update_message_metrics_with_perf_data
  - functions/src/services/message/test_update_message_metrics_without_perf_data
  - functions/src/services/message/test_delete_message
  - functions/src/services/message/test_delete_messages_for_session
  - functions/src/services/message/test_count_messages_in_session
  - functions/src/services/message/test_get_last_message
  - functions/src/services/message/test_get_messages_by_role
  - functions/src/services/message/test_calculate_totals
  - functions/src/tui/app/clear_session_is_refused_while_the_current_session_is_processing
  - functions/src/tui/app/clear_session_proceeds_when_only_another_session_is_processing
---

# Signature

`pub async fn create_message( &self, session_id: Uuid, role: String, content: String, ) -> Result<Message>`

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
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