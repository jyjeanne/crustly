---
type: Rust Function
title: default_log_level
resource: src/config/mod.rs#L1358-L1360
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/config/LoggingConfig/default/default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_log_level() -> String`

# Called by

- [default](../../../functions/src/config/LoggingConfig/default/default.md)
- [default](../../../functions/src/config/Config/default/default.md)