---
type: Rust Function
title: prefill
resource: src/llm/provider/llama_cpp.rs#L612-L659
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/prompt_cache/reusable_prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/prompt_cache/prefill_batches
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/prepare_generation
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn prefill( context: &mut LlamaContext<'_>, kv_tokens: &mut Vec<LlamaToken>, prompt_tokens: &[LlamaToken], ) -> Result<()>`

# Calls

- [reusable_prefix](../../../../../functions/src/llm/provider/prompt_cache/reusable_prefix.md)
- [prefill_batches](../../../../../functions/src/llm/provider/prompt_cache/prefill_batches.md)

# Called by

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)