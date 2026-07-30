---
type: Rust Method
title: switch_provider_to_ollama_model
resource: src/tui/app.rs#L2447-L2497
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/agent/service/AgentService/set_provider
  - functions/src/services/session/SessionService/update_session
  - functions/src/tui/app/App/switch_mode
  called_by:
  - functions/src/tui/app/App/handle_provider_switch_key
  - functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error
  - functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place
---

# Signature

`async fn switch_provider_to_ollama_model(&mut self, model: String) -> Result<()>`

# Calls

- [set_provider](../../../../../functions/src/llm/agent/service/AgentService/set_provider.md)
- [update_session](../../../../../functions/src/services/session/SessionService/update_session.md)
- [switch_mode](../../../../../functions/src/tui/app/App/switch_mode.md)

# Called by

- [handle_provider_switch_key](../../../../../functions/src/tui/app/App/handle_provider_switch_key.md)
- [switch_provider_without_ollama_feature_shows_clear_error](../../../../../functions/src/tui/app/switch_provider_without_ollama_feature_shows_clear_error.md)
- [switch_provider_with_ollama_feature_swaps_provider_in_place](../../../../../functions/src/tui/app/switch_provider_with_ollama_feature_swaps_provider_in_place.md)