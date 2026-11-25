---
type: Rust Function
title: init_debug_logging
resource: src/logging.rs#L128-L203
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/code_knowledge/CodeKnowledge/exists
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/LoggerGuard/with_guard
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/logging/init_logging
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn init_debug_logging(config: LogConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>>`

# Calls

- [exists](../../../functions/src/memory/code_knowledge/CodeKnowledge/exists.md)
- [write](../../../functions/src/memory/project/write.md)
- [build](../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [with](../../../functions/src/llm/agent/middleware/Pipeline/with.md)
- [with_guard](../../../functions/src/logging/LoggerGuard/with_guard.md)

# Called by

- [init_logging](../../../functions/src/logging/init_logging.md)