---
type: Rust Function
title: init_minimal_logging
resource: src/logging.rs#L206-L233
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/db/models/PlanTaskStatus/parse
  - functions/src/llm/agent/compaction/compact
  - functions/src/logging/LoggerGuard/empty
  called_by:
  - functions/src/logging/init_logging
---

# Signature

`fn init_minimal_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [compact](../../../functions/src/llm/agent/compaction/compact.md)
- [empty](../../../functions/src/logging/LoggerGuard/empty.md)

# Called by

- [init_logging](../../../functions/src/logging/init_logging.md)