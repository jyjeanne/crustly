---
type: Rust Method
title: read_exact
resource: src/llm/provider/gguf_metadata.rs#L187-L195
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn read_exact(&mut self, buf: &mut [u8]) -> Option<()>`

# Called by

- [read_u32](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u32.md)
- [read_u64](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64.md)
- [read_string](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_string.md)
- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)