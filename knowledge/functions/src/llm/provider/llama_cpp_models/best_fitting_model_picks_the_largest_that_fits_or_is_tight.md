---
type: Rust Function
title: best_fitting_model_picks_the_largest_that_fits_or_is_tight
resource: src/llm/provider/llama_cpp_models.rs#L2112-L2123
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/fixture_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn best_fitting_model_picks_the_largest_that_fits_or_is_tight()`

# Calls

- [best_fitting_model](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model.md)
- [fixture_model](../../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)