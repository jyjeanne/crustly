---
type: Rust Method
title: to_qwen_request
resource: src/llm/provider/qwen.rs#L623-L906
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/format_hermes_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_result
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/is_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/default_sampling
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/local_only
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/complete
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/QwenProvider/provider/stream
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn to_qwen_request(&self, request: LLMRequest) -> QwenRequest`

# Calls

- [format_hermes_tools](../../../../../../functions/src/llm/provider/qwen/QwenProvider/format_hermes_tools.md)
- [format_native_qwen_tools](../../../../../../functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_tools.md)
- [format_native_qwen_result](../../../../../../functions/src/llm/provider/qwen/QwenProvider/format_native_qwen_result.md)
- [is_local](../../../../../../functions/src/llm/provider/qwen/QwenProvider/is_local.md)
- [default_sampling](../../../../../../functions/src/llm/provider/qwen/QwenProvider/default_sampling.md)
- [local_only](../../../../../../functions/src/llm/provider/qwen/QwenProvider/local_only.md)

# Called by

- [complete](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/complete.md)
- [stream](../../../../../../functions/src/llm/provider/qwen/QwenProvider/provider/stream.md)
- [test_sampling_defaults_qwen25_coder_local](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen25_coder_local.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/qwen/test_text_sent_with_tool_results_follows_them.md)
- [test_sampling_defaults_qwen3_non_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_non_thinking.md)
- [test_sampling_defaults_qwen3_thinking](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_qwen3_thinking.md)
- [test_sampling_defaults_dashscope_omits_vendor_extensions](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions.md)
- [test_sampling_explicit_request_top_p_wins](../../../../../../functions/src/llm/provider/qwen/test_sampling_explicit_request_top_p_wins.md)
- [test_sampling_config_override_wins_over_defaults](../../../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)
- [test_sampling_defaults_unrecognized_model_name_is_conservative](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_unrecognized_model_name_is_conservative.md)