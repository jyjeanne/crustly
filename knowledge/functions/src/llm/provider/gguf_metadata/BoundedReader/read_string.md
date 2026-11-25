---
type: Rust Method
title: read_string
resource: src/llm/provider/gguf_metadata.rs#L213-L223
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64
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

`fn read_string(&mut self) -> Option<String>`

# Calls

- [read_u64](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_u64.md)
- [read_exact](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_exact.md)

# Called by

- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)