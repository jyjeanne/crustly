---
type: Rust Function
title: test_log_config_with_debug
resource: src/logging.rs#L312-L316
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
  - target: functions/src/logging/LogConfig/with_debug_mode
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn test_log_config_with_debug()`

# Calls

- [new](../../../functions/src/logging/LogConfig/new.md)
- [with_debug_mode](../../../functions/src/logging/LogConfig/with_debug_mode.md)