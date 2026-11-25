---
type: Rust Method
title: provider_name
resource: src/llm/agent/service.rs#L833-L835
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/configure_agent
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/provider_name
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/load_session
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn provider_name(&self) -> &str`

# Called by

- [configure_agent](../../../../../../functions/src/cli/configure_agent.md)
- [provider_name](../../../../../../functions/src/tui/app/App/provider_name.md)
- [create_new_session](../../../../../../functions/src/tui/app/App/create_new_session.md)
- [load_session](../../../../../../functions/src/tui/app/App/load_session.md)