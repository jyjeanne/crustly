---
type: Rust Function
title: hardware_fit
resource: src/llm/provider/llama_cpp_models.rs#L782-L793
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_fits_comfortably_under_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_tight_near_the_threshold
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_wont_fit_over_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_estimate_is_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_budget_is_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/llama_cpp_fit_tag
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn hardware_fit(estimated_memory_bytes: Option<u64>, budget_bytes: Option<u64>) -> HardwareFit`

# Called by

- [cmd_llama_cpp](../../../../../functions/src/cli/cmd_llama_cpp.md)
- [sort_by_fit](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit.md)
- [best_fitting_model](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model.md)
- [hardware_fit_reports_fits_comfortably_under_budget](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_fits_comfortably_under_budget.md)
- [hardware_fit_reports_tight_near_the_threshold](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_tight_near_the_threshold.md)
- [hardware_fit_reports_wont_fit_over_budget](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_reports_wont_fit_over_budget.md)
- [hardware_fit_is_unknown_when_estimate_is_missing](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_estimate_is_missing.md)
- [hardware_fit_is_unknown_when_budget_is_missing](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit_is_unknown_when_budget_is_missing.md)
- [llama_cpp_fit_tag](../../../../../functions/src/tui/render/llama_cpp_fit_tag.md)