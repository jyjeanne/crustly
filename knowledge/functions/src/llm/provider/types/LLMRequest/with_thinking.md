---
type: Rust Method
title: with_thinking
resource: src/llm/provider/types.rs#L179-L188
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/provider/gemini/test_thinking_config_forwarded
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_sets_temperature_and_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_sets_temperature
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/provider/types/with_thinking_zero_budget_is_noop
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/thinking_config_forces_temperature_one
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/model_routing_test/zero_budget_does_not_enable_thinking
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn with_thinking(mut self, budget_tokens: u32) -> Self`

# Called by

- [test_thinking_config_forwarded](../../../../../../functions/src/llm/provider/gemini/test_thinking_config_forwarded.md)
- [to_ollama_request_maps_thinking_and_response_format](../../../../../../functions/src/llm/provider/ollama/to_ollama_request_maps_thinking_and_response_format.md)
- [request_thinking_wins_over_configured_think](../../../../../../functions/src/llm/provider/ollama/request_thinking_wins_over_configured_think.md)
- [with_thinking_sets_temperature_and_config](../../../../../../functions/src/llm/provider/types/with_thinking_sets_temperature_and_config.md)
- [with_thinking_sets_temperature](../../../../../../functions/src/llm/provider/types/with_thinking_sets_temperature.md)
- [with_thinking_zero_budget_is_noop](../../../../../../functions/src/llm/provider/types/with_thinking_zero_budget_is_noop.md)
- [thinking_config_forces_temperature_one](../../../../../../functions/tests/model_routing_test/thinking_config_forces_temperature_one.md)
- [zero_budget_does_not_enable_thinking](../../../../../../functions/tests/model_routing_test/zero_budget_does_not_enable_thinking.md)