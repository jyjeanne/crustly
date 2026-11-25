---
type: Rust Method
title: switch_provider_to_ollama_model
resource: src/tui/app.rs#L3560-L3611
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/set_provider
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/refresh_context_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/update_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_mode
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_provider_switch_key
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn switch_provider_to_ollama_model(&mut self, model: String) -> Result<()>`

# Calls

- [set_provider](../../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [refresh_context_usage](../../../../../functions/src/tui/app/App/refresh_context_usage.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_provider_switch_key](../../../../../functions/src/tui/app/App/handle_provider_switch_key.md)
- [switch_provider_without_ollama_feature_shows_clear_error](../../../../../functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error.md)
- [switch_provider_with_ollama_feature_swaps_provider_in_place](../../../../../functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place.md)