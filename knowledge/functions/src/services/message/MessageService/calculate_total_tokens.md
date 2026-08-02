---
type: Rust Method
title: calculate_total_tokens
resource: src/services/message.rs#L185-L189
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/list_messages_for_session
  called_by:
  - functions/src/services/message/test_calculate_totals
---

# Signature

`pub async fn calculate_total_tokens(&self, session_id: Uuid) -> Result<i32>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)