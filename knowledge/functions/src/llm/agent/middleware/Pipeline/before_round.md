---
type: Rust Method
title: before_round
resource: src/llm/agent/middleware.rs#L122-L124
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Pipeline/first_decision
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/AgentRun/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn before_round(&mut self, state: &RunState) -> Decision`

# Calls

- [first_decision](../../../../../../functions/src/llm/agent/middleware/Pipeline/first_decision.md)

# Called by

- [pipeline_consults_everyone_and_the_first_decision_wins](../../../../../../functions/src/llm/agent/middleware/pipeline_consults_everyone_and_the_first_decision_wins.md)
- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)