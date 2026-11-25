---
type: Rust Method
title: default
resource: src/logging.rs#L33-L43
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/logging/LogConfig/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/test_log_config_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/test_log_dir_in_crustly_folder
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Called by

- [new](../../../../../functions/src/logging/LogConfig/new.md)
- [test_log_config_default](../../../../../functions/src/logging/test_log_config_default.md)
- [test_log_dir_in_crustly_folder](../../../../../functions/src/logging/test_log_dir_in_crustly_folder.md)