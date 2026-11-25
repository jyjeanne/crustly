---
type: Rust Function
title: dispatch_job
resource: src/llm/provider/llama_cpp.rs#L448-L516
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/run_complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/panic_to_provider_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/run_stream
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/worker_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn dispatch_job( model: &LlamaModel, context: &mut LlamaContext<'_>, kv_tokens: &mut Vec<LlamaToken>, chat_template: &Option<LlamaChatTemplate>, display_name: &str, grammar_env: &Option<ToolCallGrammarEnv>, sampling_defaults: &SamplingDefaults, seed: Option<u32>, job: InferenceJob, )`

# Calls

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [panic_to_provider_error](../../../../../functions/src/llm/provider/llama_cpp/panic_to_provider_error.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)

# Called by

- [worker_loop](../../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)