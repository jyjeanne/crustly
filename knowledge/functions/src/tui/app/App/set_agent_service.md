---
type: Rust Method
title: set_agent_service
resource: src/tui/app.rs#L812-L815
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/app/App/refresh_context_usage
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_chat
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn set_agent_service(&mut self, agent_service: Arc<AgentService>)`

# Calls

- [refresh_context_usage](../../../../../functions/src/tui/app/App/refresh_context_usage.md)

# Called by

- [cmd_chat](../../../../../functions/src/cli/cmd_chat.md)