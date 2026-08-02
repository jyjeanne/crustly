---
type: Rust Method
title: new
resource: src/llm/provider/llama_cpp.rs#L132-L219
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in
  - functions/src/llm/provider/llama_cpp/worker_loop
---

# Signature

`pub fn new(config: &crate::config::LlamaCppProviderConfig) -> Result<Self>`

# Calls

- [gpu_backend_compiled_in](../../../../../../functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in.md)
- [worker_loop](../../../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)