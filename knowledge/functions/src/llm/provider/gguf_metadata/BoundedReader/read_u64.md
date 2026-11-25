---
type: Rust Method
title: read_u64
resource: src/llm/provider/gguf_metadata.rs#L203-L207
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact
    resolved_by: tree-sitter
    confidence: exact
  called_by:
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

`fn read_u64(&mut self) -> Option<u64>`

# Calls

- [read_exact](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)

# Called by

- [read_string](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_string.md)
- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)