---
type: Rust Function
title: best_fitting_model
resource: src/llm/provider/llama_cpp_models.rs#L827-L840
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model_picks_the_largest_that_fits_or_is_tight
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_when_nothing_fits
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_for_an_empty_list
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn best_fitting_model( models: &[LocalGgufModel], budget_bytes: Option<u64>, ) -> Option<&LocalGgufModel>`

# Calls

- [hardware_fit](../../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit.md)

# Called by

- [llama_cpp_doctor_findings](../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [best_fitting_model_picks_the_largest_that_fits_or_is_tight](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_picks_the_largest_that_fits_or_is_tight.md)
- [best_fitting_model_is_none_when_nothing_fits](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_when_nothing_fits.md)
- [best_fitting_model_is_none_for_an_empty_list](../../../../../functions/src/llm/provider/llama_cpp_models/best_fitting_model_is_none_for_an_empty_list.md)