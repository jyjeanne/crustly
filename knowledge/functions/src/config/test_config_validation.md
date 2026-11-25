---
type: Rust Function
title: test_config_validation
resource: src/config/mod.rs#L1947-L1950
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/validate
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_config_validation()`

# Calls

- [default](../../../functions/src/config/Config/default/default.md)
- [validate](../../../functions/src/config/Config/validate.md)