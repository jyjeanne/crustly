---
type: Rust Function
title: verification_nudges_after_an_edit_until_a_check_passes
resource: src/llm/agent/middleware.rs#L695-L719
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
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn verification_nudges_after_an_edit_until_a_check_passes()`

# Calls

- [new](../../../../../functions/src/llm/agent/middleware/Verification/new.md)
- [state](../../../../../functions/src/llm/agent/middleware/state.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)