---
type: Rust Method
title: calculate_total_tokens
resource: src/services/message.rs#L200-L204
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/services/message/test_calculate_totals
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn calculate_total_tokens(&self, session_id: Uuid) -> Result<i32>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)