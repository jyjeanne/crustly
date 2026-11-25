---
type: Rust Method
title: default_for_test
resource: src/llm/provider/router.rs#L90-L99
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/router/all_tiers_resolve_to_non_empty_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/token_limits_ordered_correctly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/thinking_budget_by_tier
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn default_for_test() -> Self`

# Calls

- [new](../../../../../../functions/src/llm/provider/router/ModelRouter/new.md)

# Called by

- [all_tiers_resolve_to_non_empty_model](../../../../../../functions/src/llm/provider/router/all_tiers_resolve_to_non_empty_model.md)
- [token_limits_ordered_correctly](../../../../../../functions/src/llm/provider/router/token_limits_ordered_correctly.md)
- [thinking_budget_by_tier](../../../../../../functions/src/llm/provider/router/thinking_budget_by_tier.md)