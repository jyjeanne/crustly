---
type: Rust Method
title: read_u32
resource: src/llm/provider/gguf_metadata.rs#L197-L201
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
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

`fn read_u32(&mut self) -> Option<u32>`

# Calls

- [read_exact](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)

# Called by

- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)