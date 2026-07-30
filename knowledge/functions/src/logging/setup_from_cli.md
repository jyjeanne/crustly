---
type: Rust Function
title: setup_from_cli
resource: src/logging.rs#L236-L239
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/logging/LogConfig/with_debug_mode
  - functions/src/logging/init_logging
  called_by:
  - functions/src/main
---

# Signature

`pub fn setup_from_cli(debug: bool) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [with_debug_mode](../../../functions/src/logging/LogConfig/with_debug_mode.md)
- [init_logging](../../../functions/src/logging/init_logging.md)

# Called by

- [main](../../../functions/src/main.md)