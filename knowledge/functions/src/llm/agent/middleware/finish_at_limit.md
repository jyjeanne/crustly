---
type: Rust Function
title: finish_at_limit
resource: src/llm/agent/middleware.rs#L397-L408
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/middleware/Limits/roundmiddleware/before_round
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn finish_at_limit(what: &str) -> Decision`

# Called by

- [before_round](../../../../../functions/src/llm/agent/middleware/Limits/roundmiddleware/before_round.md)