---
type: Rust Method
title: update_message_usage
resource: src/services/message.rs#L96-L113
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/get_message_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/message/MessageRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/message/MessageRepository/update
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
  - target: functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_update_message_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/test_calculate_totals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_message_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>`

# Calls

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)
- [new](../../../../../functions/src/db/repository/message/MessageRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/message/MessageRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [test_update_message_usage](../../../../../functions/src/services/message/test_update_message_usage.md)
- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)