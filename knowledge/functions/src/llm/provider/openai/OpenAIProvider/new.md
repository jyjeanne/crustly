---
type: Rust Method
title: new
resource: src/llm/provider/openai.rs#L70-L86
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/provider/factory/try_create_openai
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_openai_provider_creation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/default_auth_style_still_sends_bearer
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_supported_models
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_supports_vision_detection
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_calculate_cost
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_calculate_cost_current_models_are_not_free
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(api_key: String) -> Self`

# Calls

- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)

# Called by

- [try_create_openai](../../../../../../functions/src/llm/provider/factory/try_create_openai.md)
- [test_openai_provider_creation](../../../../../../functions/src/llm/provider/openai/test_openai_provider_creation.md)
- [default_auth_style_still_sends_bearer](../../../../../../functions/src/llm/provider/openai/default_auth_style_still_sends_bearer.md)
- [test_supported_models](../../../../../../functions/src/llm/provider/openai/test_supported_models.md)
- [test_context_window](../../../../../../functions/src/llm/provider/openai/test_context_window.md)
- [test_supports_vision_detection](../../../../../../functions/src/llm/provider/openai/test_supports_vision_detection.md)
- [test_new_fields_forwarded_to_openai_request](../../../../../../functions/src/llm/provider/openai/test_new_fields_forwarded_to_openai_request.md)
- [test_text_sent_with_tool_results_follows_them](../../../../../../functions/src/llm/provider/openai/test_text_sent_with_tool_results_follows_them.md)
- [test_calculate_cost](../../../../../../functions/src/llm/provider/openai/test_calculate_cost.md)
- [test_calculate_cost_current_models_are_not_free](../../../../../../functions/src/llm/provider/openai/test_calculate_cost_current_models_are_not_free.md)
- [test_calculate_cost_unknown_model_returns_zero](../../../../../../functions/src/llm/provider/openai/test_calculate_cost_unknown_model_returns_zero.md)