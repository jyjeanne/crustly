---
type: Rust Method
title: new
resource: src/logging.rs#L48-L50
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/logging/LogConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/logging/setup_from_cli
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/test_log_config_with_debug
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/test_log_config_builder
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Calls

- [default](../../../../functions/src/logging/LogConfig/default/default.md)

# Called by

- [setup_from_cli](../../../../functions/src/logging/setup_from_cli.md)
- [test_log_config_with_debug](../../../../functions/src/logging/test_log_config_with_debug.md)
- [test_log_config_builder](../../../../functions/src/logging/test_log_config_builder.md)