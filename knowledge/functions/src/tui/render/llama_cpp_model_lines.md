---
type: Rust Function
title: llama_cpp_model_lines
resource: src/tui/render.rs#L2632-L2726
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/format_param_count
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/render/render_llama_cpp_models
    resolved_by: tree-sitter
    confidence: exact
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
---

# Signature

`fn llama_cpp_model_lines( model: &super::llama_cpp_download::LlamaCppModelSummary, is_selected: bool, budget_bytes: Option<u64>, ) -> Vec<Line<'static>>`

# Calls

- [push](../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [format_param_count](../../../../functions/src/tui/render/format_param_count.md)

# Called by

- [render_llama_cpp_models](../../../../functions/src/tui/render/render_llama_cpp_models.md)
- [llama_cpp_model_lines_prefers_display_name_over_the_raw_filename](../../../../functions/src/tui/render/llama_cpp_model_lines_prefers_display_name_over_the_raw_filename.md)
- [llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently](../../../../functions/src/tui/render/llama_cpp_model_lines_labels_a_paired_and_an_unpaired_mmproj_entry_differently.md)
- [llama_cpp_model_lines_shows_a_detail_line_only_when_selected](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_detail_line_only_when_selected.md)
- [llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_a_fit_tag_when_budget_is_known.md)
- [llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_wont_fit_when_estimate_exceeds_budget.md)
- [llama_cpp_model_lines_shows_the_context_length_the_estimate_used](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_the_context_length_the_estimate_used.md)
- [llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_without_the_gguf_management_feature.md)
- [llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown](../../../../functions/src/tui/render/llama_cpp_model_lines_shows_no_fit_tag_when_budget_is_unknown.md)