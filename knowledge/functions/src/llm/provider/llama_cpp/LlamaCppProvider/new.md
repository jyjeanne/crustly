---
type: Rust Method
title: new
resource: src/llm/provider/llama_cpp.rs#L131-L218
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/worker_loop
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_llama_cpp
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/llama_cpp/new_reports_model_not_found_for_a_missing_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/llama_cpp_download/build_llama_cpp_provider
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(config: &crate::config::LlamaCppProviderConfig) -> Result<Self>`

# Calls

- [gpu_backend_compiled_in](../../../../../../functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in.md)
- [worker_loop](../../../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)

# Called by

- [try_create_llama_cpp](../../../../../../functions/src/llm/provider/factory/try_create_llama_cpp.md)
- [new_reports_model_not_found_for_a_missing_path](../../../../../../functions/src/llm/provider/llama_cpp/new_reports_model_not_found_for_a_missing_path.md)
- [build_llama_cpp_provider](../../../../../../functions/src/tui/llama_cpp_download/build_llama_cpp_provider.md)