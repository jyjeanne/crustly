---
type: Rust Function
title: parse_ollama_manifest
resource: src/llm/provider/llama_cpp_models.rs#L421-L491
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/all
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
  - target: functions/src/llm/provider/llama_cpp_models/list_ollama_models
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse_ollama_manifest( manifest_path: &Path, manifests_root: &Path, blobs_root: &Path, ) -> Option<LocalGgufModel>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [all](../../../../../functions/src/memory/MemoryStore/all.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)
- [estimate_default_context_length](../../../../../functions/src/llm/provider/llama_cpp_models/estimate_default_context_length.md)
- [estimate_memory_bytes](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes.md)

# Called by

- [list_ollama_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_ollama_models.md)