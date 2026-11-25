---
type: Rust Function
title: hardware_fit_label
resource: src/cli/mod.rs#L556-L564
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_llama_cpp
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/cli/hardware_fit_label_matches_the_documented_wording
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn hardware_fit_label(fit: crate::llm::provider::llama_cpp_models::HardwareFit) -> &'static str`

# Called by

- [cmd_llama_cpp](../../../functions/src/cli/cmd_llama_cpp.md)
- [hardware_fit_label_matches_the_documented_wording](../../../functions/src/cli/hardware_fit_label_matches_the_documented_wording.md)