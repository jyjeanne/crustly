---
type: Rust Method
title: resolve
resource: src/llm/provider/router.rs#L42-L48
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/router/all_tiers_resolve_to_non_empty_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/all_tiers_resolve_to_valid_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn resolve(&self, tier: ModelTier) -> (&str, &str)`

# Called by

- [all_tiers_resolve_to_non_empty_model](../../../../../../functions/src/llm/provider/router/all_tiers_resolve_to_non_empty_model.md)
- [all_tiers_resolve_to_valid_model](../../../../../../functions/tests/model_routing_test/all_tiers_resolve_to_valid_model.md)