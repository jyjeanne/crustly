---
type: Rust Function
title: all_tiers_resolve_to_non_empty_model
resource: src/llm/provider/router.rs#L113-L120
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/router/ModelRouter/default_for_test
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/router/ModelRouter/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn all_tiers_resolve_to_non_empty_model()`

# Calls

- [default_for_test](../../../../../functions/src/llm/provider/router/ModelRouter/default_for_test.md)
- [resolve](../../../../../functions/src/llm/provider/router/ModelRouter/resolve.md)