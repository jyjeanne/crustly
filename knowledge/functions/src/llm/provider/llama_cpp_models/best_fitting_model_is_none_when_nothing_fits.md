---
type: Rust Function
title: best_fitting_model_is_none_when_nothing_fits
resource: src/llm/provider/llama_cpp_models.rs#L2126-L2129
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/fixture_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn best_fitting_model_is_none_when_nothing_fits()`

# Calls

- [fixture_model](../../../../../functions/src/llm/provider/llama_cpp_models/fixture_model.md)
- [best_fitting_model](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model.md)