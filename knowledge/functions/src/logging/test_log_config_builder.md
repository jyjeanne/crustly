---
type: Rust Function
title: test_log_config_builder
resource: src/logging.rs#L319-L328
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/logging/LogConfig/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/LogConfig/with_log_level
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/LogConfig/with_console_output
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/LogConfig/with_log_prefix
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_log_config_builder()`

# Calls

- [new](../../../functions/src/logging/LogConfig/new.md)
- [with_log_level](../../../functions/src/logging/LogConfig/with_log_level.md)
- [with_console_output](../../../functions/src/logging/LogConfig/with_console_output.md)
- [with_log_prefix](../../../functions/src/logging/LogConfig/with_log_prefix.md)