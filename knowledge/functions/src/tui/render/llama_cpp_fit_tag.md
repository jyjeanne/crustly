---
type: Rust Function
title: llama_cpp_fit_tag
resource: src/tui/render.rs#L2585-L2605
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp_models/hardware_fit
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_fit_tag( estimated_memory_bytes: Option<u64>, estimated_memory_context_length: Option<u64>, budget_bytes: Option<u64>, ) -> Option<(String, Color)>`

# Calls

- [hardware_fit](../../../../functions/src/llm/provider/llama_cpp_models/hardware_fit.md)