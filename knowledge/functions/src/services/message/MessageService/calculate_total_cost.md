---
type: Rust Method
title: calculate_total_cost
resource: src/services/message.rs#L192-L196
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/services/message/MessageService/list_messages_for_session
  called_by:
  - functions/src/services/message/test_calculate_totals
---

# Signature

`pub async fn calculate_total_cost(&self, session_id: Uuid) -> Result<f64>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)