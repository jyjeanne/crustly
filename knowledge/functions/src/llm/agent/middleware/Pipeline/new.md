---
type: Rust Method
title: new
resource: src/llm/agent/middleware.rs#L99-L101
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/round_pipeline
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new() -> Self`

# Called by

- [pipeline_consults_everyone_and_the_first_decision_wins](../../../../../../functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins.md)
- [round_pipeline](../../../../../../functions/src/llm/agent/service/AgentService/round_pipeline.md)