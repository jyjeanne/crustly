---
type: Rust Function
title: llama_cpp_model_fixture
resource: src/tui/render.rs#L3470-L3488
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/render/llama_cpp_model_lines_prefers_display_name_over_the_raw_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_a_detail_line_only_when_selected
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_the_context_length_the_estimate_used
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_models_dialog_shows_display_name_not_the_raw_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_model_fixture( name: &str, ) -> super::super::llama_cpp_download::LlamaCppModelSummary`

# Called by

- [llama_cpp_model_lines_prefers_display_name_over_the_raw_filename](../../../../functions/src/tui/render/llama_cpp_model_lines_prefers_display_name_over_the_raw_filename.md)
- [llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently](../../../../functions/src/tui/render/llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently.md)
- [llama_cpp_model_lines_shows_a_detail_line_only_when_selected](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_detail_line_only_when_selected.md)
- [llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known.md)
- [llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget.md)
- [llama_cpp_model_lines_shows_the_context_length_the_estimate_used](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_the_context_length_the_estimate_used.md)
- [llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature.md)
- [llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown.md)
- [llama_cpp_models_dialog_shows_display_name_not_the_raw_path](../../../../functions/src/tui/render/llama_cpp_models_dialog_shows_display_name_not_the_raw_path.md)