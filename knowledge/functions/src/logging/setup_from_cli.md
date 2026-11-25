---
type: Rust Function
title: setup_from_cli
resource: src/logging.rs#L236-L239
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/logging/LogConfig/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/logging/LogConfig/with_debug_mode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_logging
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/main
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn setup_from_cli(debug: bool) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [new](../../../functions/src/logging/LogConfig/new.md)
- [with_debug_mode](../../../functions/src/logging/LogConfig/with_debug_mode.md)
- [init_logging](../../../functions/src/logging/init_logging.md)

# Called by

- [main](../../../functions/src/main.md)