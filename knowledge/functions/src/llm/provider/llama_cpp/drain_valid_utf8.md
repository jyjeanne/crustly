---
type: Rust Function
title: drain_valid_utf8
resource: src/llm/provider/llama_cpp.rs#L1113-L1127
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8_multiple_tokens_reassemble_correctly
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8_never_panics_on_arbitrary_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8_full_ascii_chunk
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8_empty_buffer_returns_none
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn drain_valid_utf8(buffer: &mut Vec<u8>) -> Option<String>`

# Called by

- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [drain_valid_utf8_multiple_tokens_reassemble_correctly](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_multiple_tokens_reassemble_correctly.md)
- [drain_valid_utf8_never_panics_on_arbitrary_bytes](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_never_panics_on_arbitrary_bytes.md)
- [drain_valid_utf8_full_ascii_chunk](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_full_ascii_chunk.md)
- [drain_valid_utf8_empty_buffer_returns_none](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_empty_buffer_returns_none.md)
- [drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence.md)