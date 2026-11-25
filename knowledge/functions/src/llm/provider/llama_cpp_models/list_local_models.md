---
type: Rust Function
title: list_local_models
resource: src/llm/provider/llama_cpp_models.rs#L192-L261
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/tui/error/ErrorInfo/with_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/read_gguf_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/estimate_default_context_length
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/list_all_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn list_local_models(models_dir: &Path) -> Result<Vec<LocalGgufModel>>`

# Calls

- [with_context](../../../../../functions/src/tui/error/ErrorInfo/with_context.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)
- [estimate_default_context_length](../../../../../functions/src/llm/provider/llama_cpp_models/estimate_default_context_length.md)
- [estimate_memory_bytes](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes.md)

# Called by

- [list_all_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_all_local_models.md)
- [list_local_models_on_nonexistent_dir_returns_empty_not_error](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_on_nonexistent_dir_returns_empty_not_error.md)
- [list_local_models_only_lists_gguf_files](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models_only_lists_gguf_files.md)