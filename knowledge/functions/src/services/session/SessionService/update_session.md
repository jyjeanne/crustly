---
type: Rust Method
title: update_session
resource: src/services/session.rs#L70-L83
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/App/handle_event
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/complete_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/switch_provider_to_ollama_model
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_session(&self, session: &Session) -> Result<()>`

# Calls

- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [handle_event](../../../../../functions/src/tui/app/App/handle_event.md)
- [create_new_session](../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../../functions/src/tui/app/App/load_session.md)
- [complete_response](../../../../../functions/src/tui/app/App/complete_response.md)
- [switch_provider_to_ollama_model](../../../../../functions/src/tui/app/App/switch_provider_to_ollama_model.md)