---
type: Rust Method
title: calculate_delay
resource: src/db/retry.rs#L61-L68
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/retry/retry_db_operation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/retry_db_sqlx
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/test_calculate_delay
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn calculate_delay(&self, attempt: u32) -> Duration`

# Called by

- [retry_db_operation](../../../../../functions/src/db/retry/retry_db_operation.md)
- [retry_db_sqlx](../../../../../functions/src/db/retry/retry_db_sqlx.md)
- [test_calculate_delay](../../../../../functions/src/db/retry/test_calculate_delay.md)