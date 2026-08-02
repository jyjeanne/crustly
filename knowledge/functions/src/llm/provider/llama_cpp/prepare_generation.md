---
type: Rust Function
title: prepare_generation
resource: src/llm/provider/llama_cpp.rs#L530-L574
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/build_prompt
  - functions/src/config/secrets/SecretString/len
  - functions/src/llm/provider/llama_cpp/build_sampler
  called_by:
  - functions/src/llm/provider/llama_cpp/run_complete
  - functions/src/llm/provider/llama_cpp/run_stream
---

# Signature

`fn prepare_generation( model: &LlamaModel, context: &mut LlamaContext<'_>, chat_template: &Option<LlamaChatTemplate>, sampling_defaults: &SamplingDefaults, default_seed: Option<u32>, request: &LLMRequest, ) -> Result<PreparedGeneration>`

# Calls

- [build_prompt](../../../../../functions/src/llm/provider/llama_cpp/build_prompt.md)
- [len](../../../../../functions/src/config/secrets/SecretString/len.md)
- [build_sampler](../../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)