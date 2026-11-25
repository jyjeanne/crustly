---
type: Rust Method
title: default
resource: src/config/mod.rs#L1324-L1328
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/config/default_db_path
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/config/test_database_config_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_db_path](../../../../../functions/src/config/default_db_path.md)

# Called by

- [test_database_config_default](../../../../../functions/src/config/test_database_config_default.md)