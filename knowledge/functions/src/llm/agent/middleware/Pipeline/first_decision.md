---
type: Rust Method
title: first_decision
resource: src/llm/agent/middleware.rs#L108-L120
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/Pipeline/before_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/on_tool_calls
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/on_final_answer
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn first_decision( &mut self, mut ask: impl FnMut(&mut dyn RoundMiddleware) -> Decision, ) -> Decision`

# Called by

- [before_round](../../../../../../functions/src/llm/agent/middleware/Pipeline/before_round.md)
- [on_tool_calls](../../../../../../functions/src/llm/agent/middleware/Pipeline/on_tool_calls.md)
- [on_final_answer](../../../../../../functions/src/llm/agent/middleware/Pipeline/on_final_answer.md)