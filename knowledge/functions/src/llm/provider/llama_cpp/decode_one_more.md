---
type: Rust Function
title: decode_one_more
resource: src/llm/provider/llama_cpp.rs#L579-L592
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/provider/llama_cpp/run_complete
  - functions/src/llm/provider/llama_cpp/run_stream
---

# Signature

`fn decode_one_more( context: &mut LlamaContext<'_>, batch: &mut LlamaBatch<'_>, token: LlamaToken, pos: i32, ) -> Result<()>`

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)