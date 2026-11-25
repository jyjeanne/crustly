---
type: Rust Method
title: from_tag
resource: src/llm/provider/gguf_metadata.rs#L127-L144
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/BoundedReader/read_value
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn from_tag(tag: u32) -> Option<Self>`

# Called by

- [read_value](../../../../../../functions/src/llm/provider/gguf_metadata/BoundedReader/read_value.md)
- [parse](../../../../../../functions/src/llm/provider/gguf_metadata/parse.md)