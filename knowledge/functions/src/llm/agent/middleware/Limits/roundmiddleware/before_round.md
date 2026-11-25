---
type: Rust Method
title: before_round
resource: src/llm/agent/middleware.rs#L411-L426
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/finish_at_limit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn before_round(&mut self, state: &RunState) -> Decision`

# Calls

- [finish_at_limit](../../../../../../../functions/src/llm/agent/middleware/finish_at_limit.md)