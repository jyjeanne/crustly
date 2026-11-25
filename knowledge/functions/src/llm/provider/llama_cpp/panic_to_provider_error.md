---
type: Rust Function
title: panic_to_provider_error
resource: src/llm/provider/llama_cpp.rs#L519-L523
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/panic_message
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/dispatch_job
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn panic_to_provider_error(payload: &(dyn std::any::Any + Send)) -> ProviderError`

# Calls

- [panic_message](../../../../../functions/src/llm/provider/llama_cpp/panic_message.md)

# Called by

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)