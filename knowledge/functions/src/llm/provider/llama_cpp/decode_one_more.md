---
type: Rust Function
title: decode_one_more
resource: src/llm/provider/llama_cpp.rs#L664-L686
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

`fn decode_one_more( context: &mut LlamaContext<'_>, kv_tokens: &mut Vec<LlamaToken>, batch: &mut LlamaBatch<'_>, token: LlamaToken, pos: i32, ) -> Result<()>`

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)