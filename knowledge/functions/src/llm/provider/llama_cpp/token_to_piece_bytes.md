---
type: Rust Function
title: token_to_piece_bytes
resource: src/llm/provider/llama_cpp.rs#L1036-L1047
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp/run_complete
  - functions/src/llm/provider/llama_cpp/run_stream
---

# Signature

`fn token_to_piece_bytes( model: &LlamaModel, token: LlamaToken, ) -> std::result::Result<Vec<u8>, String>`

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)