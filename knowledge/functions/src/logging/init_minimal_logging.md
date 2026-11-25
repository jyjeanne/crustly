---
type: Rust Function
title: init_minimal_logging
resource: src/logging.rs#L206-L233
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/LoggerGuard/empty
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/logging/init_logging
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn init_minimal_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [with](../../../functions/src/llm/agent/middleware/Pipeline/with.md)
- [empty](../../../functions/src/logging/LoggerGuard/empty.md)

# Called by

- [init_logging](../../../functions/src/logging/init_logging.md)