---
type: Rust Function
title: gpu_backend_compiled_in
resource: src/llm/provider/llama_cpp.rs#L104-L113
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/LlamaCppProvider/new
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in_is_false_in_this_cpu_only_test_build
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`const fn gpu_backend_compiled_in() -> bool`

# Called by

- [new](../../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)
- [gpu_backend_compiled_in_is_false_in_this_cpu_only_test_build](../../../../../functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in_is_false_in_this_cpu_only_test_build.md)