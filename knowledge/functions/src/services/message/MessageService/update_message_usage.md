---
type: Rust Method
title: update_message_usage
resource: src/services/message.rs#L95-L112
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/message/MessageService/get_message_required
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool
  - functions/src/services/message/test_update_message_usage
  - functions/src/services/message/test_calculate_totals
---

# Signature

`pub async fn update_message_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>`

# Calls

- [get_message_required](../../../../../functions/src/services/message/MessageService/get_message_required.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [create_then_update_survives_a_file_backed_wal_pool](../../../../../functions/src/services/message/create_then_update_survives_a_file_backed_wal_pool.md)
- [test_update_message_usage](../../../../../functions/src/services/message/test_update_message_usage.md)
- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)