---
type: Rust Method
title: max_output_tokens
resource: src/llm/provider/router.rs#L51-L57
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/router/token_limits_ordered_correctly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/model_routing_test/token_limits_ordered_by_tier
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn max_output_tokens(&self, tier: ModelTier) -> u32`

# Called by

- [token_limits_ordered_correctly](../../../../../../functions/src/llm/provider/router/token_limits_ordered_correctly.md)
- [token_limits_ordered_by_tier](../../../../../../functions/tests/model_routing_test/token_limits_ordered_by_tier.md)