---
type: Rust Method
title: set_provider
resource: src/llm/agent/service.rs#L823-L830
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch
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

`pub fn set_provider(&mut self, provider: Arc<dyn Provider>)`

# Called by

- [calibration_learns_from_the_provider_and_resets_on_switch](../../../../../../functions/src/llm/agent/service/calibration_learns_from_the_provider_and_resets_on_switch.md)
- [handle_event](../../../../../../functions/src/tui/app/App/handle_event.md)
- [switch_provider_to_ollama_model](../../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)