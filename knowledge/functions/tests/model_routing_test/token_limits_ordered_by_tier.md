---
type: Rust Function
title: token_limits_ordered_by_tier
resource: tests/model_routing_test.rs#L41-L52
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/router/ModelRouter/max_output_tokens
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn token_limits_ordered_by_tier()`

# Calls

- [default](../../../functions/src/llm/provider/router/ModelRouter/default/default.md)
- [max_output_tokens](../../../functions/src/llm/provider/router/ModelRouter/max_output_tokens.md)