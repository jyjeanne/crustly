---
type: Rust Method
title: on_tool_calls
resource: src/llm/agent/middleware.rs#L126-L128
generated:
  by: okf-rs/0.7.0
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

`pub fn on_tool_calls(&mut self, state: &RunState) -> Decision`

# Calls

- [first_decision](../../../../../../functions/src/llm/agent/middleware/Pipeline/first_decision.md)

# Called by

- [run](../../../../../../functions/src/llm/agent/service/run/AgentRun/run.md)