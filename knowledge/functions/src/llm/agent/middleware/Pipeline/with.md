---
type: Rust Method
title: with
resource: src/llm/agent/middleware.rs#L103-L106
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_debug_logging
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/logging/init_minimal_logging
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with(mut self, middleware: impl RoundMiddleware + 'static) -> Self`

# Called by

- [pipeline_consults_everyone_and_the_first_decision_wins](../../../../../../functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)
- [init_debug_logging](../../../../../../functions/src/logging/init_debug_logging.md)
- [init_minimal_logging](../../../../../../functions/src/logging/init_minimal_logging.md)