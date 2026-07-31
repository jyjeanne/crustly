---
type: Rust Method
title: set_provider
resource: src/llm/agent/service.rs#L637-L639
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/app/App/switch_provider_to_ollama_model
---

# Signature

`pub fn set_provider(&mut self, provider: Arc<dyn Provider>)`

# Called by

- [switch_provider_to_ollama_model](../../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)