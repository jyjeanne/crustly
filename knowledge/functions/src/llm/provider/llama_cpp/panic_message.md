---
type: Rust Function
title: panic_message
resource: src/llm/provider/llama_cpp.rs#L509-L517
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp/panic_to_provider_error
---

# Signature

`fn panic_message(payload: &(dyn std::any::Any + Send)) -> String`

# Called by

- [panic_to_provider_error](../../../../../functions/src/llm/provider/llama_cpp/panic_to_provider_error.md)