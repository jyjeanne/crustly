---
type: Rust Function
title: parse
resource: src/llm/provider/gguf_metadata.rs#L269-L377
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/ValueType/from_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/ggml_type_to_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn parse(reader: &mut BoundedReader<impl Read>) -> Option<GgufMetadata>`

# Calls

- [read_exact](../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)
- [read_u32](../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32.md)
- [contains](../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [read_u64](../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64.md)
- [read_string](../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_string.md)
- [from_tag](../../../../../functions/src/llm/provider/gguf_metadata/ValueType/from_tag.md)
- [read_value](../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [ggml_type_to_string](../../../../../functions/src/llm/provider/gguf_metadata/ggml_type_to_string.md)