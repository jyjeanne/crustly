---
type: Rust Method
title: retry_after
resource: src/utils/retry.rs#L15-L17
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/utils/retry/retry
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn retry_after(&self) -> Option<Duration>`

# Called by

- [retry](../../../../../functions/src/utils/retry/retry.md)