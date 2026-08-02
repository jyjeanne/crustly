---
type: Rust Function
title: run_complete
resource: src/llm/provider/llama_cpp.rs#L599-L743
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/llama_cpp/prepare_generation
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/llm/provider/llama_cpp/token_to_piece_bytes
  - functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler
  - functions/src/llm/provider/llama_cpp/decode_one_more
  - functions/src/llm/provider/tool_call_recovery/tool_call_from_content
  called_by:
  - functions/src/llm/provider/llama_cpp/dispatch_job
---

# Signature

`fn run_complete( model: &LlamaModel, context: &mut LlamaContext<'_>, chat_template: &Option<LlamaChatTemplate>, display_name: &str, grammar_env: &Option<ToolCallGrammarEnv>, sampling_defaults: &SamplingDefaults, default_seed: Option<u32>, request: LLMRequest, ) -> Result<LLMResponse>`

# Calls

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)
- [is_empty](../../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [token_to_piece_bytes](../../../../../functions/src/llm/provider/llama_cpp/token_to_piece_bytes.md)
- [maybe_swap_to_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler.md)
- [decode_one_more](../../../../../functions/src/llm/provider/llama_cpp/decode_one_more.md)
- [tool_call_from_content](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)

# Called by

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)