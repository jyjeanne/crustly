---
type: Rust Function
title: parses_architecture_name_context_length_and_chat_template
resource: src/llm/provider/gguf_metadata.rs#L593-L612
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
  - target: functions/src/llm/provider/gguf_metadata/push_string_kv
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/push_u32_kv
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

`fn parses_architecture_name_context_length_and_chat_template()`

# Calls

- [header](../../../../../functions/src/llm/provider/gguf_metadata/header.md)
- [push_string_kv](../../../../../functions/src/llm/provider/gguf_metadata/push_string_kv.md)
- [push_u32_kv](../../../../../functions/src/llm/provider/gguf_metadata/push_u32_kv.md)
- [write_temp_gguf](../../../../../functions/src/llm/provider/gguf_metadata/write_temp_gguf.md)
- [read_gguf_metadata](../../../../../functions/src/llm/provider/gguf_metadata/read_gguf_metadata.md)