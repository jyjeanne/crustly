---
type: Rust Method
title: provider_name
resource: src/tui/app.rs#L503-L505
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/App/set_llama_cpp_config
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/llama_cpp_model_details
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn provider_name(&self) -> &str`

# Calls

- [provider_name](../../../../../functions/src/llm/agent/service/AgentService/provider_name.md)

# Called by

- [set_llama_cpp_config](../../../../../functions/src/tui/app/App/set_llama_cpp_config.md)
- [llama_cpp_model_details](../../../../../functions/src/tui/app/App/llama_cpp_model_details.md)
- [switch_provider_with_ollama_feature_swaps_provider_in_place](../../../../../functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place.md)
- [llama_cpp_switch_finished_with_error_reports_failure_without_swapping](../../../../../functions/src/tui/app/llama_cpp_switch_finished_with_error_reports_failure_without_swapping.md)
- [llama_cpp_switch_finished_swaps_provider_in_place](../../../../../functions/src/tui/app/llama_cpp_switch_finished_swaps_provider_in_place.md)
- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)