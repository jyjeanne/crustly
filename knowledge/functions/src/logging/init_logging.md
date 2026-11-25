---
type: Rust Function
title: init_logging
resource: src/logging.rs#L117-L125
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_minimal_logging
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/logging/setup_from_cli
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn init_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [init_debug_logging](../../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../../functions/src/logging/init_minimal_logging.md)

# Called by

- [setup_from_cli](../../../functions/src/logging/setup_from_cli.md)