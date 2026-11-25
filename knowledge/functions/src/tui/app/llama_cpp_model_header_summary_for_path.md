---
type: Rust Function
title: llama_cpp_model_header_summary_for_path
resource: src/tui/app.rs#L379-L394
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/read_gguf_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn llama_cpp_model_header_summary_for_path( path: &std::path::Path, ) -> (Option<String>, Option<u64>, bool)`

# Calls

- [read_gguf_metadata](../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)
- [quantization_hint_from_filename](../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_from_filename.md)