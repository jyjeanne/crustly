---
type: Rust Module
title: router
resource: src/llm/provider/router.rs#L1-L144
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/serde-deserialize-serialize
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [ModelTier](../../../../classes/src/llm/provider/router/ModelTier.md)
- [ModelRouter](../../../../classes/src/llm/provider/router/ModelRouter.md)
- [new](../../../../functions/src/llm/provider/router/ModelRouter/new.md)
- [resolve](../../../../functions/src/llm/provider/router/ModelRouter/resolve.md)
- [max_output_tokens](../../../../functions/src/llm/provider/router/ModelRouter/max_output_tokens.md)
- [context_window](../../../../functions/src/llm/provider/router/ModelRouter/context_window.md)
- [thinking_budget](../../../../functions/src/llm/provider/router/ModelRouter/thinking_budget.md)
- [default_anthropic](../../../../functions/src/llm/provider/router/ModelRouter/default_anthropic.md)
- [default_for_test](../../../../functions/src/llm/provider/router/ModelRouter/default_for_test.md)
- [default](../../../../functions/src/llm/provider/router/ModelRouter/default/default.md)
- [all_tiers_resolve_to_non_empty_model](../../../../functions/src/llm/provider/router/all_tiers_resolve_to_non_empty_model.md)
- [token_limits_ordered_correctly](../../../../functions/src/llm/provider/router/token_limits_ordered_correctly.md)
- [thinking_budget_by_tier](../../../../functions/src/llm/provider/router/thinking_budget_by_tier.md)

# Imports

- `serde::{Deserialize, Serialize}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)