---
type: Rust Function
title: truncation_at_several_points_returns_none_not_a_panic
resource: src/llm/provider/gguf_metadata.rs#L777-L792
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/gguf_metadata/header
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/push_string_kv
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

`fn truncation_at_several_points_returns_none_not_a_panic()`

# Calls

- [header](../../../../../functions/src/llm/provider/gguf_metadata/header.md)
- [push_string_kv](../../../../../functions/src/llm/provider/gguf_metadata/push_string_kv.md)
- [write_temp_gguf](../../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)