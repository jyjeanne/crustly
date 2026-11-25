---
type: Rust Method
title: default
resource: src/config/crabrace.rs#L46-L53
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/crabrace/default_base_url
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/default_auto_update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/config/crabrace/default_update_interval
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/crabrace/test_default_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/crabrace/test_create_integration
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/crabrace/test_health_check
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/Config/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_base_url](../../../../../../functions/src/config/crabrace/default_base_url.md)
- [default_auto_update](../../../../../../functions/src/config/crabrace/default_auto_update.md)
- [default_update_interval](../../../../../../functions/src/config/crabrace/default_update_interval.md)

# Called by

- [test_default_config](../../../../../../functions/src/config/crabrace/test_default_config.md)
- [test_create_integration](../../../../../../functions/src/config/crabrace/test_create_integration.md)
- [test_health_check](../../../../../../functions/src/config/crabrace/test_health_check.md)
- [default](../../../../../../functions/src/config/Config/default/default.md)