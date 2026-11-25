---
type: Rust Function
title: pipeline_consults_everyone_and_the_first_decision_wins
resource: src/llm/agent/middleware.rs#L756-L779
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Pipeline/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/Pipeline/with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/before_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/state
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn pipeline_consults_everyone_and_the_first_decision_wins()`

# Calls

- [new](../../../../../functions/src/llm/agent/middleware/Pipeline/new.md)
- [with](../../../../../functions/src/llm/agent/middleware/Pipeline/with.md)
- [before_round](../../../../../functions/src/llm/agent/middleware/Pipeline/before_round.md)
- [state](../../../../../functions/src/llm/agent/middleware/state.md)