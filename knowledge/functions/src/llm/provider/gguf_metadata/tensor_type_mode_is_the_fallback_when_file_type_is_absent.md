---
type: Rust Function
title: tensor_type_mode_is_the_fallback_when_file_type_is_absent
resource: src/llm/provider/gguf_metadata.rs#L678-L694
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
  - target: functions/src/llm/provider/gguf_metadata/push_tensor
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

`fn tensor_type_mode_is_the_fallback_when_file_type_is_absent()`

# Calls

- [header](../../../../../functions/src/llm/provider/gguf_metadata/header.md)
- [push_tensor](../../../../../functions/src/llm/provider/gguf_metadata/push_tensor.md)
- [write_temp_gguf](../../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)