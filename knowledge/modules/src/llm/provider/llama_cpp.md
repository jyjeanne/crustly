---
type: Rust Module
title: llama_cpp
resource: src/llm/provider/llama_cpp.rs#L1-L1857
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/super-error-providererror-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-r-trait-provider-providerstream
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-tool-call-recovery-commits-to-an-offered-tool-call-maybe-tool-call-json-tool-call-from-content
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-types
    resolved_by: tree-sitter
    confidence: exact
  - target: external/async-trait-async-trait
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-context-params-llamacontextparams
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-context-llamacontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-llama-backend-llamabackend
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-llama-batch-llamabatch
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-model-params-llamamodelparams
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-model-llamachatmessage-llamachattemplate-llamamodel
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-sampling-llamasampler
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-token-llamatoken
    resolved_by: tree-sitter
    confidence: exact
  - target: external/llama-cpp-2-vocab-llamavocab
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-num-nonzerou32
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tokio-sync-mpsc-oneshot
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-prompt-cache-prefill-batches-reusable-prefix
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [SamplingDefaults](../../../../classes/src/llm/provider/llama_cpp/SamplingDefaults.md)
- [default](../../../../functions/src/llm/provider/llama_cpp/SamplingDefaults/default/default.md)
- [InferenceJob](../../../../classes/src/llm/provider/llama_cpp/InferenceJob.md)
- [gpu_backend_compiled_in](../../../../functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in.md)
- [LlamaCppProvider](../../../../classes/src/llm/provider/llama_cpp/LlamaCppProvider.md)
- [new](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/new.md)
- [WorkerInit](../../../../classes/src/llm/provider/llama_cpp/WorkerInit.md)
- [worker_loop](../../../../functions/src/llm/provider/llama_cpp/worker_loop.md)
- [dispatch_job](../../../../functions/src/llm/provider/llama_cpp/dispatch_job.md)
- [panic_to_provider_error](../../../../functions/src/llm/provider/llama_cpp/panic_to_provider_error.md)
- [panic_message](../../../../functions/src/llm/provider/llama_cpp/panic_message.md)
- [PreparedGeneration](../../../../classes/src/llm/provider/llama_cpp/PreparedGeneration.md)
- [prepare_generation](../../../../functions/src/llm/provider/llama_cpp/prepare_generation.md)
- [prefill](../../../../functions/src/llm/provider/llama_cpp/prefill.md)
- [decode_one_more](../../../../functions/src/llm/provider/llama_cpp/decode_one_more.md)
- [run_complete](../../../../functions/src/llm/provider/llama_cpp/run_complete.md)
- [run_stream](../../../../functions/src/llm/provider/llama_cpp/run_stream.md)
- [drain_valid_utf8](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8.md)
- [token_to_piece_bytes](../../../../functions/src/llm/provider/llama_cpp/token_to_piece_bytes.md)
- [build_sampler](../../../../functions/src/llm/provider/llama_cpp/build_sampler.md)
- [build_grammar_env](../../../../functions/src/llm/provider/llama_cpp/build_grammar_env.md)
- [build_grammar_env](../../../../functions/src/llm/provider/llama_cpp/build_grammar_env-2.md)
- [try_build_constrained_sampler](../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler.md)
- [try_build_constrained_sampler](../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler-2.md)
- [maybe_swap_to_constrained_sampler](../../../../functions/src/llm/provider/llama_cpp/maybe_swap_to_constrained_sampler.md)
- [tool_instructions_block](../../../../functions/src/llm/provider/llama_cpp/tool_instructions_block.md)
- [merged_system_prompt](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt.md)
- [build_prompt](../../../../functions/src/llm/provider/llama_cpp/build_prompt.md)
- [complete](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/complete.md)
- [stream](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/stream.md)
- [supports_streaming](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/supports_streaming.md)
- [supports_tools](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/supports_tools.md)
- [supports_vision](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/supports_vision.md)
- [name](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/name.md)
- [default_model](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/default_model.md)
- [supported_models](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/supported_models.md)
- [validate_model](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/validate_model.md)
- [context_window](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/context_window.md)
- [calculate_cost](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/provider/calculate_cost.md)
- [fmt](../../../../functions/src/llm/provider/llama_cpp/LlamaCppProvider/std-fmt-debug/fmt.md)
- [new_reports_model_not_found_for_a_missing_path](../../../../functions/src/llm/provider/llama_cpp/new_reports_model_not_found_for_a_missing_path.md)
- [display_name_defaults_to_the_file_stem](../../../../functions/src/llm/provider/llama_cpp/display_name_defaults_to_the_file_stem.md)
- [sampling_defaults_match_documented_values](../../../../functions/src/llm/provider/llama_cpp/sampling_defaults_match_documented_values.md)
- [build_sampler_seed_offset_changes_the_resolved_seed](../../../../functions/src/llm/provider/llama_cpp/build_sampler_seed_offset_changes_the_resolved_seed.md)
- [gpu_backend_compiled_in_is_false_in_this_cpu_only_test_build](../../../../functions/src/llm/provider/llama_cpp/gpu_backend_compiled_in_is_false_in_this_cpu_only_test_build.md)
- [drain_valid_utf8_full_ascii_chunk](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_full_ascii_chunk.md)
- [drain_valid_utf8_empty_buffer_returns_none](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_empty_buffer_returns_none.md)
- [drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_holds_back_an_incomplete_multibyte_sequence.md)
- [drain_valid_utf8_multiple_tokens_reassemble_correctly](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_multiple_tokens_reassemble_correctly.md)
- [drain_valid_utf8_never_panics_on_arbitrary_bytes](../../../../functions/src/llm/provider/llama_cpp/drain_valid_utf8_never_panics_on_arbitrary_bytes.md)
- [bash_tool](../../../../functions/src/llm/provider/llama_cpp/bash_tool.md)
- [merged_system_prompt_with_neither_is_none](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_with_neither_is_none.md)
- [merged_system_prompt_system_only_is_unchanged](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_system_only_is_unchanged.md)
- [merged_system_prompt_empty_tools_list_behaves_like_none](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_empty_tools_list_behaves_like_none.md)
- [merged_system_prompt_tools_only_still_produces_instructions](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_tools_only_still_produces_instructions.md)
- [merged_system_prompt_combines_system_and_tools_with_system_first](../../../../functions/src/llm/provider/llama_cpp/merged_system_prompt_combines_system_and_tools_with_system_first.md)
- [tool_instructions_block_names_every_offered_tool](../../../../functions/src/llm/provider/llama_cpp/tool_instructions_block_names_every_offered_tool.md)
- [test_grammar_env](../../../../functions/src/llm/provider/llama_cpp/test_grammar_env.md)
- [try_build_constrained_sampler_succeeds_for_a_valid_tool_schema](../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_succeeds_for_a_valid_tool_schema.md)
- [try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay](../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_does_not_panic_on_an_arbitrary_token_replay.md)
- [try_build_constrained_sampler_without_the_feature_is_always_none](../../../../functions/src/llm/provider/llama_cpp/try_build_constrained_sampler_without_the_feature_is_always_none.md)

# Imports

- `super::error::{ProviderError, Result}`
- `super::r#trait::{Provider, ProviderStream}`
- `super::tool_call_recovery::{
    commits_to_an_offered_tool_call, maybe_tool_call_json, tool_call_from_content,
}`
- `super::types::*`
- `async_trait::async_trait`
- `llama_cpp_2::context::params::LlamaContextParams`
- `llama_cpp_2::context::LlamaContext`
- `llama_cpp_2::llama_backend::LlamaBackend`
- `llama_cpp_2::llama_batch::LlamaBatch`
- `llama_cpp_2::model::params::LlamaModelParams`
- `llama_cpp_2::model::{LlamaChatMessage, LlamaChatTemplate, LlamaModel}`
- `llama_cpp_2::sampling::LlamaSampler`
- `llama_cpp_2::token::LlamaToken`
- `llama_cpp_2::vocab::LlamaVocab`
- `std::num::NonZeroU32`
- `std::path::PathBuf`
- `tokio::sync::{mpsc, oneshot}`
- `super::prompt_cache::{prefill_batches, reusable_prefix}`
- `super::*`

# Member of

- [crustly](../../../../packages/crustly.md)