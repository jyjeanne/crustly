---
type: Rust Function
title: sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown
resource: src/llm/provider/llama_cpp_models.rs#L2078-L2091
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/fixture_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown()`

# Calls

- [sort_by_fit](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit.md)
- [fixture_model](../../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)