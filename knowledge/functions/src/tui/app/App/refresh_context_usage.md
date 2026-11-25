---
type: Rust Method
title: refresh_context_usage
resource: src/tui/app.rs#L539-L541
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/set_agent_service
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_provider_to_ollama_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn refresh_context_usage(&mut self)`

# Calls

- [context_usage_preview](../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)

# Called by

- [set_agent_service](../../../../../functions/src/tui/app/App/set_agent_service.md)
- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [switch_provider_to_ollama_model](../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)