---
type: Rust Module
title: message
resource: src/services/message.rs#L1-L596
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-db-models-message-repository-messagerepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-servicecontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-sessionservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-llm-provider-perfmetrics
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [MessageService](../../../classes/src/services/message/MessageService.md)
- [new](../../../functions/src/services/message/MessageService/new.md)
- [create_message](../../../functions/src/services/message/MessageService/create_message.md)
- [get_message](../../../functions/src/services/message/MessageService/get_message.md)
- [get_message_required](../../../functions/src/services/message/MessageService/get_message_required.md)
- [list_messages_for_session](../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [update_message](../../../functions/src/services/message/MessageService/update_message.md)
- [update_message_usage](../../../functions/src/services/message/MessageService/update_message_usage.md)
- [update_message_tool_log](../../../functions/src/services/message/MessageService/update_message_tool_log.md)
- [update_message_metrics](../../../functions/src/services/message/MessageService/update_message_metrics.md)
- [delete_message](../../../functions/src/services/message/MessageService/delete_message.md)
- [delete_messages_for_session](../../../functions/src/services/message/MessageService/delete_messages_for_session.md)
- [count_messages_in_session](../../../functions/src/services/message/MessageService/count_messages_in_session.md)
- [get_last_message](../../../functions/src/services/message/MessageService/get_last_message.md)
- [get_messages_by_role](../../../functions/src/services/message/MessageService/get_messages_by_role.md)
- [calculate_total_tokens](../../../functions/src/services/message/MessageService/calculate_total_tokens.md)
- [calculate_total_cost](../../../functions/src/services/message/MessageService/calculate_total_cost.md)
- [create_test_service](../../../functions/src/services/message/create_test_service.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [test_create_message](../../../functions/src/services/message/test_create_message.md)
- [test_get_message](../../../functions/src/services/message/test_get_message.md)
- [test_list_messages_for_session](../../../functions/src/services/message/test_list_messages_for_session.md)
- [test_update_message_usage](../../../functions/src/services/message/test_update_message_usage.md)
- [test_update_message_metrics_with_perf_data](../../../functions/src/services/message/test_update_message_metrics_with_perf_data.md)
- [test_update_message_metrics_without_perf_data](../../../functions/src/services/message/test_update_message_metrics_without_perf_data.md)
- [test_delete_message](../../../functions/src/services/message/test_delete_message.md)
- [test_delete_messages_for_session](../../../functions/src/services/message/test_delete_messages_for_session.md)
- [test_count_messages_in_session](../../../functions/src/services/message/test_count_messages_in_session.md)
- [test_get_last_message](../../../functions/src/services/message/test_get_last_message.md)
- [test_get_messages_by_role](../../../functions/src/services/message/test_get_messages_by_role.md)
- [test_calculate_totals](../../../functions/src/services/message/test_calculate_totals.md)

# Imports

- `crate::db::{models::Message, repository::MessageRepository}`
- `crate::services::ServiceContext`
- `anyhow::{Context, Result}`
- `chrono::Utc`
- `uuid::Uuid`
- `super::*`
- `crate::services::SessionService`
- `crate::db::Database`
- `crate::llm::provider::PerfMetrics`

# Member of

- [crustly](../../../packages/crustly.md)