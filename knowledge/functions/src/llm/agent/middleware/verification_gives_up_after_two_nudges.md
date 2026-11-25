---
type: Rust Function
title: verification_gives_up_after_two_nudges
resource: src/llm/agent/middleware.rs#L722-L734
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Verification/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/state
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verification_gives_up_after_two_nudges()`

# Calls

- [new](../../../../../functions/src/llm/agent/middleware/Verification/new.md)
- [state](../../../../../functions/src/llm/agent/middleware/state.md)