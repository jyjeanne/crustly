---
type: Rust Function
title: push_string_kv
resource: src/llm/provider/gguf_metadata.rs#L545-L549
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
  - target: functions/src/llm/provider/gguf_metadata/a_string_array_between_two_kv_pairs_is_skipped_correctly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/gguf_metadata/truncation_at_several_points_returns_none_not_a_panic
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn push_string_kv(buf: &mut Vec<u8>, key: &str, value: &str)`

# Called by

- [parses_architecture_name_context_length_and_chat_template](../../../../../functions/src/llm/provider/gguf_metadata/parses_architecture_name_context_length_and_chat_template.md)
- [a_string_array_between_two_kv_pairs_is_skipped_correctly](../../../../../functions/src/llm/provider/gguf_metadata/a_string_array_between_two_kv_pairs_is_skipped_correctly.md)
- [truncation_at_several_points_returns_none_not_a_panic](../../../../../functions/src/llm/provider/gguf_metadata/truncation_at_several_points_returns_none_not_a_panic.md)