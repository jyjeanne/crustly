---
type: Rust Function
title: init_debug_logging
resource: src/logging.rs#L128-L203
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/logging/LoggerGuard/with_guard
  called_by:
  - functions/src/logging/init_logging
---

# Signature

`fn init_debug_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [is_empty](../../../functions/src/config/secrets/SecretString/is_empty.md)
- [with_guard](../../../functions/src/logging/LoggerGuard/with_guard.md)

# Called by

- [init_logging](../../../functions/src/logging/init_logging.md)