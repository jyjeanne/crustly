---
type: Rust Function
title: sort_by_fit
resource: src/llm/provider/llama_cpp_models.rs#L802-L820
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/rank
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_largest_first_within_a_category
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn sort_by_fit(models: &mut [LocalGgufModel], budget_bytes: Option<u64>)`

# Calls

- [hardware_fit](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit.md)
- [rank](../../../../../functions/src/llm/provider/llama_cpp_models/rank.md)

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown.md)
- [sort_by_fit_orders_largest_first_within_a_category](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_largest_first_within_a_category.md)