---
type: Rust Method
title: default
resource: src/db/retry.rs#L30-L37
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/test_retry_config_defaults
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/test_retry_success_immediate
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/retry/test_retry_non_retryable_error
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [insert](../../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [test_retry_config_defaults](../../../../../../functions/src/db/retry/test_retry_config_defaults.md)
- [test_retry_success_immediate](../../../../../../functions/src/db/retry/test_retry_success_immediate.md)
- [test_retry_non_retryable_error](../../../../../../functions/src/db/retry/test_retry_non_retryable_error.md)