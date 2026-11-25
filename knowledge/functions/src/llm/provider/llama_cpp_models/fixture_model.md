---
type: Rust Function
title: fixture_model
resource: src/llm/provider/llama_cpp_models.rs#L2029-L2046
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_largest_first_within_a_category
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model_picks_the_largest_that_fits_or_is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_when_nothing_fits
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn fixture_model(name: &str, estimated_memory_bytes: Option<u64>) -> LocalGgufModel`

# Called by

- [merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_prefers_the_first_part_with_a_parsed_header_on_a_tie.md)
- [merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header](../../../../../functions/src/llm/provider/llama_cpp_models/merge_split_gguf_groups_falls_back_to_part_one_when_no_part_has_a_header.md)
- [sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_fits_before_tight_before_wont_fit_before_unknown.md)
- [sort_by_fit_orders_largest_first_within_a_category](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit_orders_largest_first_within_a_category.md)
- [best_fitting_model_picks_the_largest_that_fits_or_is_tight](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_picks_the_largest_that_fits_or_is_tight.md)
- [best_fitting_model_is_none_when_nothing_fits](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_when_nothing_fits.md)