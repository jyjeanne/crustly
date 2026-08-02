---
type: Rust Method
title: with_retry
resource: src/tui/error.rs#L155-L160
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/error/test_error_info_with_retry
---

# Signature

`pub fn with_retry(mut self, retry_count: u32, next_retry: DateTime<Utc>) -> Self`

# Called by

- [test_error_info_with_retry](../../../../../functions/src/tui/error/test_error_info_with_retry.md)