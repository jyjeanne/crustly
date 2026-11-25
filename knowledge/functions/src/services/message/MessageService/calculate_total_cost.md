---
type: Rust Method
title: calculate_total_cost
resource: src/services/message.rs#L207-L211
generated:
  by: okf-rs/0.7.0
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

`pub async fn calculate_total_cost(&self, session_id: Uuid) -> Result<f64>`

# Calls

- [list_messages_for_session](../../../../../functions/src/services/message/MessageService/list_messages_for_session.md)

# Called by

- [test_calculate_totals](../../../../../functions/src/services/message/test_calculate_totals.md)