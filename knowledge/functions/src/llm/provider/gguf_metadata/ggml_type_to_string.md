---
type: Rust Function
title: ggml_type_to_string
resource: src/llm/provider/gguf_metadata.rs#L419-L438
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/parse
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn ggml_type_to_string(code: u32) -> Option<String>`

# Called by

- [parse](../../../../../functions/src/llm/provider/gguf_metadata/parse.md)