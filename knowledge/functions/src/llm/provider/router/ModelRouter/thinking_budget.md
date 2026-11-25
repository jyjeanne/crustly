---
type: Rust Method
title: thinking_budget
resource: src/llm/provider/router.rs#L68-L74
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/router/thinking_budget_by_tier
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn thinking_budget(&self, tier: ModelTier) -> Option<u32>`

# Called by

- [thinking_budget_by_tier](../../../../../../functions/src/llm/provider/router/thinking_budget_by_tier.md)