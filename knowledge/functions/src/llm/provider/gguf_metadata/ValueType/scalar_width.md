---
type: Rust Method
title: scalar_width
resource: src/llm/provider/gguf_metadata.rs#L148-L156
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn scalar_width(self) -> Option<usize>`

# Called by

- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)