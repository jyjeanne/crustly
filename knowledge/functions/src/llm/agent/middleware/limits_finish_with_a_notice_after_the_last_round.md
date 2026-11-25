---
type: Rust Function
title: limits_finish_with_a_notice_after_the_last_round
resource: src/llm/agent/middleware.rs#L654-L667
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Limits/default/default
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

`fn limits_finish_with_a_notice_after_the_last_round()`

# Calls

- [default](../../../../../functions/src/llm/agent/middleware/Limits/default/default.md)
- [state](../../../../../functions/src/llm/agent/middleware/state.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)