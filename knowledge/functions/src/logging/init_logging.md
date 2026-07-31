---
type: Rust Function
title: init_logging
resource: src/logging.rs#L117-L125
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/logging/init_debug_logging
  - functions/src/logging/init_minimal_logging
  called_by:
  - functions/src/logging/setup_from_cli
---

# Signature

`pub fn init_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [init_debug_logging](../../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../../functions/src/logging/init_minimal_logging.md)

# Called by

- [setup_from_cli](../../../functions/src/logging/setup_from_cli.md)