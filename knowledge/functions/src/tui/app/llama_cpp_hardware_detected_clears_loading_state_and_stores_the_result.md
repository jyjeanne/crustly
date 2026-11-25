---
type: Rust Function
title: llama_cpp_hardware_detected_clears_loading_state_and_stores_the_result
resource: src/tui/app.rs#L5375-L5393
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/llama_cpp_download/HardwareSummary/budget_bytes
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn llama_cpp_hardware_detected_clears_loading_state_and_stores_the_result()`

# Calls

- [handle_event](../../../../functions/src/tui/app/App/handle_event.md)
- [budget_bytes](../../../../functions/src/tui/llama_cpp_download/HardwareSummary/budget_bytes.md)