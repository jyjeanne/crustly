---
type: Rust Function
title: maybe_swap_to_constrained_sampler
resource: src/llm/provider/llama_cpp.rs#L1249-L1281
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call
  called_by:
  - functions/src/llm/provider/llama_cpp/run_complete
  - functions/src/llm/provider/llama_cpp/run_stream
---

# Signature

`fn maybe_swap_to_constrained_sampler( text: &str, swap_possible: bool, grammar_swap_attempted: &mut bool, grammar_env: &Option<ToolCallGrammarEnv>, offered_tools: &[Tool], generated_tokens: &[LlamaToken], sampling_defaults: &SamplingDefaults, request: &LLMRequest, default_seed: Option<u32>, sampler: &mut LlamaSampler, )`

# Calls

- [commits_to_an_offered_tool_call](../../../../../functions/src/llm/provider/tool_call_recovery/commits_to_an_offered_tool_call.md)

# Called by

- [run_complete](../../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../../functions/src/llm/provider/llama_cpp/run_stream.md)