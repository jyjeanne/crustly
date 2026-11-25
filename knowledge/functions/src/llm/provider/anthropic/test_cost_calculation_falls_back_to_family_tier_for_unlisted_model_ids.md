---
type: Rust Function
title: test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids
resource: src/llm/provider/anthropic.rs#L540-L559
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

`fn test_cost_calculation_falls_back_to_family_tier_for_unlisted_model_ids()`

# Calls

- [new](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/new.md)
- [calculate_cost](../../../../../functions/src/llm/provider/anthropic/AnthropicProvider/provider/calculate_cost.md)