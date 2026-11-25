---
type: Rust Function
title: run_stream
resource: src/llm/provider/llama_cpp.rs#L847-L1107
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/llama_cpp/prepare_generation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/token_to_piece_bytes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/drain_valid_utf8
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/llama_cpp/decode_one_more
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/tool_call_recovery/tool_call_from_content
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/llama_cpp/dispatch_job
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn run_stream( model: &LlamaModel, context: &mut LlamaContext<'_>, kv_tokens: &mut Vec<LlamaToken>, chat_template: &Option<LlamaChatTemplate>, display_name: &str, grammar_env: &Option<ToolCallGrammarEnv>, sampling_defaults: &SamplingDefaults, default_seed: Option<u32>, request: LLMRequest, events_tx: &mpsc::UnboundedSender<Result<StreamEvent>>, )`

# Calls

- [prepare_generation](../../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)
- [token_to_piece_bytes](../../../../../functions/src/llm/provider/llama_cpp/token_to_piece_bytes.md)
- [drain_valid_utf8](../../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8.md)
- [maybe_tool_call_json](../../../../../functions/src/llm/provider/tool_call_recovery/maybe_tool_call_json.md)
- [maybe_swap_to_constrained_sampler](../../../../../functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler.md)
- [decode_one_more](../../../../../functions/src/llm/provider/llama_cpp/decode_one_more.md)
- [tool_call_from_content](../../../../../functions/src/llm/provider/tool_call_recovery/tool_call_from_content.md)

# Called by

- [dispatch_job](../../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)