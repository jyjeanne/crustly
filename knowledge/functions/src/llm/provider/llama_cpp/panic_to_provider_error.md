---
type: Rust Function
title: panic_to_provider_error
resource: src/llm/provider/llama_cpp.rs#L499-L503
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/panic_message
  called_by:
  - functions/src/llm/provider/llama_cpp/dispatch_job
---

# Signature

`fn panic_to_provider_error(payload: &(dyn std::any::Any + Send)) -> ProviderError`

# Calls

- [panic_message](../../../../../functions/src/llm/provider/llama_cpp/panic_message.md)

# Called by

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)