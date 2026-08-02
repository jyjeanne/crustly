---
type: Rust Function
title: dispatch_job
resource: src/llm/provider/llama_cpp.rs#L443-L496
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/run_complete
  - functions/src/llm/provider/llama_cpp/panic_to_provider_error
  - functions/src/llm/provider/llama_cpp/run_stream
  called_by:
  - functions/src/llm/provider/llama_cpp/worker_loop
---

# Signature

`fn dispatch_job( model: &LlamaModel, context: &mut LlamaContext<'_>, chat_template: &Option<LlamaChatTemplate>, display_name: &str, grammar_env: &Option<ToolCallGrammarEnv>, sampling_defaults: &SamplingDefaults, seed: Option<u32>, job: InferenceJob, )`

# Calls

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [panic_to_provider_error](../../../../../functions/src/llm/provider/llama_cpp/panic_to_provider_error.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)

# Called by

- [worker_loop](../../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)