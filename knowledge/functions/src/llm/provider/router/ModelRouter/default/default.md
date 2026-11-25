---
type: Rust Method
title: default
resource: src/llm/provider/router.rs#L103-L105
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/default_anthropic
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/model_routing_test/all_tiers_resolve_to_valid_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/token_limits_ordered_by_tier
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn default() -> Self`

# Calls

- [default_anthropic](../../../../../../../functions/src/llm/provider/router/ModelRouter/default_anthropic.md)

# Called by

- [all_tiers_resolve_to_valid_model](../../../../../../../functions/tests/model_routing_test/all_tiers_resolve_to_valid_model.md)
- [token_limits_ordered_by_tier](../../../../../../../functions/tests/model_routing_test/token_limits_ordered_by_tier.md)