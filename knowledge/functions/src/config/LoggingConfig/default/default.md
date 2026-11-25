---
type: Rust Method
title: default
resource: src/config/mod.rs#L1350-L1355
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_log_level
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/test_logging_config_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_log_level](../../../../../functions/src/config/default_log_level.md)

# Called by

- [test_logging_config_default](../../../../../functions/src/config/test_logging_config_default.md)