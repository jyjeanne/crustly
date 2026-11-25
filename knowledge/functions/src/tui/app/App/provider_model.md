---
type: Rust Method
title: provider_model
resource: src/tui/app.rs#L508-L510
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/provider_model
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn provider_model(&self) -> &str`

# Calls

- [provider_model](../../../../../functions/src/llm/agent/service/AgentService/provider_model.md)

# Called by

- [switch_provider_with_ollama_feature_swaps_provider_in_place](../../../../../functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place.md)
- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)