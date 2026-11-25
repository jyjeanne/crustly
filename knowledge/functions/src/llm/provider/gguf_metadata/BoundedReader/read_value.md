---
type: Rust Method
title: read_value
resource: src/llm/provider/gguf_metadata.rs#L230-L266
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/ValueType/from_tag
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/ValueType/scalar_width
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_value(&mut self, ty: ValueType) -> Option<DecodedValue>`

# Calls

- [read_string](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_string.md)
- [read_u32](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32.md)
- [from_tag](../../../../../../functions/src/llm/provider/gguf_metadata/ValueType/from_tag.md)
- [read_u64](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64.md)
- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [scalar_width](../../../../../../functions/src/llm/provider/gguf_metadata/ValueType/scalar_width.md)
- [read_exact](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)

# Called by

- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)