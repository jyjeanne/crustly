---
type: Rust Function
title: available_space_at
resource: src/llm/provider/llama_cpp_models.rs#L934-L950
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/llama_cpp_doctor_findings
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/download_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/available_space_at_a_real_directory_returns_a_plausible_value_or_none
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/available_space_at_walks_up_to_the_nearest_existing_ancestor
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub(crate) fn available_space_at(path: &Path) -> Option<u64>`

# Called by

- [llama_cpp_doctor_findings](../../../../../functions/src/cli/llama_cpp_doctor_findings.md)
- [download_model](../../../../../functions/src/llm/provider/llama_cpp_models/download_model.md)
- [available_space_at_a_real_directory_returns_a_plausible_value_or_none](../../../../../functions/src/llm/provider/llama_cpp_models/available_space_at_a_real_directory_returns_a_plausible_value_or_none.md)
- [available_space_at_walks_up_to_the_nearest_existing_ancestor](../../../../../functions/src/llm/provider/llama_cpp_models/available_space_at_walks_up_to_the_nearest_existing_ancestor.md)