---
type: Rust Function
title: zero_budget_does_not_enable_thinking
resource: tests/model_routing_test.rs#L31-L37
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/types/LLMRequest/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/LLMRequest/with_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn zero_budget_does_not_enable_thinking()`

# Calls

- [new](../../../functions/src/llm/provider/types/LLMRequest/new.md)
- [with_thinking](../../../functions/src/llm/provider/types/LLMRequest/with_thinking.md)