---
type: Rust Function
title: test_cost_calculation
resource: src/llm/provider/anthropic.rs#L527-L537
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/anthropic/AnthropicProvider/provider/calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn test_cost_calculation()`

# Calls

- [new](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/calculate_cost.md)