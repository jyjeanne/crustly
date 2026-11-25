---
type: Rust Method
title: dashscope_intl
resource: src/llm/provider/qwen.rs#L123-L125
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/provider/qwen/QwenProvider/with_base_url
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/provider/factory/try_create_qwen
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_qwen_provider_creation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_supported_models
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_context_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_calculate_cost_cloud
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_calculate_cost_unknown_cloud_model_returns_zero
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn dashscope_intl(api_key: String) -> Self`

# Calls

- [with_base_url](../../../../../../functions/src/llm/provider/qwen/QwenProvider/with_base_url.md)

# Called by

- [try_create_qwen](../../../../../../functions/src/llm/provider/factory/try_create_qwen.md)
- [test_qwen_provider_creation](../../../../../../functions/src/llm/provider/qwen/test_qwen_provider_creation.md)
- [test_supported_models](../../../../../../functions/src/llm/provider/qwen/test_supported_models.md)
- [test_context_window](../../../../../../functions/src/llm/provider/qwen/test_context_window.md)
- [test_calculate_cost_cloud](../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_cloud.md)
- [test_calculate_cost_unknown_cloud_model_returns_zero](../../../../../../functions/src/llm/provider/qwen/test_calculate_cost_unknown_cloud_model_returns_zero.md)
- [test_sampling_defaults_dashscope_omits_vendor_extensions](../../../../../../functions/src/llm/provider/qwen/test_sampling_defaults_dashscope_omits_vendor_extensions.md)
- [test_sampling_config_override_wins_over_defaults](../../../../../../functions/src/llm/provider/qwen/test_sampling_config_override_wins_over_defaults.md)