---
type: Rust Function
title: quantization_hint_from_filename
resource: src/llm/provider/llama_cpp_models.rs#L166-L172
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_recognizes_common_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_none_for_unrecognized_filename
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp_models/quantization_hint_is_case_insensitive
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/llama_cpp_model_header_summary_for_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn quantization_hint_from_filename(filename: &str) -> Option<String>`

# Calls

- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)

# Called by

- [quantization_hint_recognizes_common_tags](../../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_recognizes_common_tags.md)
- [quantization_hint_none_for_unrecognized_filename](../../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_none_for_unrecognized_filename.md)
- [quantization_hint_is_case_insensitive](../../../../../functions/src/llm/provider/llama_cpp_models/quantization_hint_is_case_insensitive.md)
- [llama_cpp_model_header_summary_for_path](../../../../../functions/src/tui/app/llama_cpp_model_header_summary_for_path.md)