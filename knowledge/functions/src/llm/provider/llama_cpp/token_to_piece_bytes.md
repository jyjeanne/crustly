---
type: Rust Function
title: token_to_piece_bytes
resource: src/llm/provider/llama_cpp.rs#L1132-L1134
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/provider/llama_cpp/run_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn token_to_piece_bytes(vocab: &LlamaVocab<'_>, token: LlamaToken) -> Vec<u8>`

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)