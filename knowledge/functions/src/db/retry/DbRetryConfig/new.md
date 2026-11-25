---
type: Rust Method
title: new
resource: src/db/retry.rs#L42-L48
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/retry/test_retry_success_after_retries
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/test_retry_max_attempts_exceeded
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(max_attempts: u32, initial_delay: Duration) -> Self`

# Called by

- [test_retry_success_after_retries](../../../../../functions/src/db/retry/test_retry_success_after_retries.md)
- [test_retry_max_attempts_exceeded](../../../../../functions/src/db/retry/test_retry_max_attempts_exceeded.md)