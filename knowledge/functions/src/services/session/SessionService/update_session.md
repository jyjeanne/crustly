---
type: Rust Method
title: update_session
resource: src/services/session.rs#L72-L85
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/tui/app/App/create_new_session
  - functions/src/tui/app/App/load_session
  - functions/src/tui/app/App/complete_response
  - functions/src/tui/app/App/switch_provider_to_ollama_model
---

# Signature

`pub async fn update_session(&self, session: &Session) -> Result<()>`

# Called by

- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [switch_provider_to_ollama_model](../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)