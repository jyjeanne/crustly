---
type: Rust Function
title: estimate_memory_bytes
resource: src/llm/provider/gguf_metadata.rs#L495-L531
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/bits_per_weight
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_covers_weights_only_when_geometry_is_missing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_includes_kv_cache_when_geometry_is_present
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_is_none_without_parameter_count_or_quantization
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/list_local_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn estimate_memory_bytes( metadata: &GgufMetadata, context_length: u64, ) -> Option<MemoryEstimate>`

# Calls

- [bits_per_weight](../../../../../functions/src/llm/provider/gguf_metadata/bits_per_weight.md)

# Called by

- [estimate_memory_bytes_covers_weights_only_when_geometry_is_missing](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_covers_weights_only_when_geometry_is_missing.md)
- [estimate_memory_bytes_includes_kv_cache_when_geometry_is_present](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_includes_kv_cache_when_geometry_is_present.md)
- [estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_falls_back_to_head_count_without_gqa_kv_heads.md)
- [estimate_memory_bytes_is_none_without_parameter_count_or_quantization](../../../../../functions/src/llm/provider/gguf_metadata/estimate_memory_bytes_is_none_without_parameter_count_or_quantization.md)
- [list_local_models](../../../../../functions/src/llm/provider/llama_cpp_models/list_local_models.md)
- [parse_ollama_manifest](../../../../../functions/src/llm/provider/llama_cpp_models/parse_ollama_manifest.md)