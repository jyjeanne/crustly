---
type: Rust Function
title: test_preset_configs
resource: src/utils/retry.rs#L411-L423
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/utils/retry/RetryConfig/database
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/utils/retry/RetryConfig/api
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_preset_configs()`

# Calls

- [database](../../../../functions/src/utils/retry/RetryConfig/database.md)
- [api](../../../../functions/src/utils/retry/RetryConfig/api.md)