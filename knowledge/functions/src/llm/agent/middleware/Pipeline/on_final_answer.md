---
type: Rust Method
title: on_final_answer
resource: src/llm/agent/middleware.rs#L136-L138
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Pipeline/first_decision
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn on_final_answer(&mut self, state: &RunState) -> Decision`

# Calls

- [first_decision](../../../../../../functions/src/llm/agent/middleware/Pipeline/first_decision.md)

# Called by

- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)