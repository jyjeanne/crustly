---
type: Rust Function
title: loop_guard_stops_a_repeated_write_on_the_second_call
resource: src/llm/agent/middleware.rs#L588-L603
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/LoopGuard/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/call
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/state
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn loop_guard_stops_a_repeated_write_on_the_second_call()`

# Calls

- [new](../../../../../functions/src/llm/agent/middleware/LoopGuard/new.md)
- [call](../../../../../functions/src/llm/agent/middleware/call.md)
- [state](../../../../../functions/src/llm/agent/middleware/state.md)