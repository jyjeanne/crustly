---
type: Rust Function
title: oversized_declared_array_length_is_rejected
resource: src/llm/provider/gguf_metadata.rs#L810-L819
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/write_temp_gguf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/read_gguf_metadata
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn oversized_declared_array_length_is_rejected()`

# Calls

- [header](../../../../../functions/src/llm/provider/gguf_metadata/header.md)
- [write_temp_gguf](../../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)