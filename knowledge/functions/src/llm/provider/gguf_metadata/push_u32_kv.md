---
type: Rust Function
title: push_u32_kv
resource: src/llm/provider/gguf_metadata.rs#L553-L557
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/general_file_type_maps_to_the_precise_quantization_string
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_u32_kv(buf: &mut Vec<u8>, key: &str, value: u32)`

# Called by

- [parses_architecture_name_context_length_and_chat_template](../../../../../functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template.md)
- [general_file_type_maps_to_the_precise_quantization_string](../../../../../functions/src/llm/provider/gguf_metadata/general_file_type_maps_to_the_precise_quantization_string.md)