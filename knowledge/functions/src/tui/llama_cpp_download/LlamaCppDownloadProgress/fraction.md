---
type: Rust Method
title: fraction
resource: src/tui/llama_cpp_download.rs#L73-L76
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/tui/llama_cpp_download/download_progress_fraction
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/llama_cpp_download/download_progress_fraction_unknown_total_is_none
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn fraction(&self) -> Option<f64>`

# Called by

- [download_progress_fraction](../../../../../functions/src/tui/llama_cpp_download/download_progress_fraction.md)
- [download_progress_fraction_unknown_total_is_none](../../../../../functions/src/tui/llama_cpp_download/download_progress_fraction_unknown_total_is_none.md)