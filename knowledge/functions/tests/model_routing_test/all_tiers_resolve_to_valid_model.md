---
type: Rust Function
title: all_tiers_resolve_to_valid_model
resource: tests/model_routing_test.rs#L10-L17
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
  - target: functions/src/llm/provider/router/ModelRouter/resolve
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn all_tiers_resolve_to_valid_model()`

# Calls

- [default](../../../functions/src/llm/provider/router/ModelRouter/default/default.md)
- [resolve](../../../functions/src/llm/provider/router/ModelRouter/resolve.md)