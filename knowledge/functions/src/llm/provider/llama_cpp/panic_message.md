---
type: Rust Function
title: panic_message
resource: src/llm/provider/llama_cpp.rs#L529-L537
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/panic_to_provider_error
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn panic_message(payload: &(dyn std::any::Any + Send)) -> String`

# Called by

- [panic_to_provider_error](../../../../../functions/src/llm/provider/llama_cpp/panic_to_provider_error.md)