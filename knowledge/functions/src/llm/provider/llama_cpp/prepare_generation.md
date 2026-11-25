---
type: Rust Function
title: prepare_generation
resource: src/llm/provider/llama_cpp.rs#L550-L597
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/build_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/prefill
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/run_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn prepare_generation( model: &LlamaModel, context: &mut LlamaContext<'_>, kv_tokens: &mut Vec<LlamaToken>, chat_template: &Option<LlamaChatTemplate>, sampling_defaults: &SamplingDefaults, default_seed: Option<u32>, request: &LLMRequest, ) -> Result<PreparedGeneration>`

# Calls

- [build_sampler](../../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)
- [prefill](../../../../../functions/src/llm/provider/llama_cpp/prefill.md)

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)