---
type: Rust Function
title: quantization_hint_from_filename
resource: src/llm/provider/llama_cpp_models.rs#L64-L70
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/tui/app/quantization_hint_for_path
---

# Signature

`pub fn quantization_hint_from_filename(filename: &str) -> Option<String>`

# Called by

- [quantization_hint_for_path](../../../../../functions/src/tui/app/quantization_hint_for_path.md)