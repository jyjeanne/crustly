---
type: Rust Function
title: rank
resource: src/llm/provider/llama_cpp_models.rs#L803-L810
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/sort_by_fit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn rank(fit: HardwareFit) -> u8`

# Called by

- [sort_by_fit](../../../../../functions/src/llm/provider/llama_cpp_models/sort_by_fit.md)