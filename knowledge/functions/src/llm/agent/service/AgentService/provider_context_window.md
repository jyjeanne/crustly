---
type: Rust Method
title: provider_context_window
resource: src/llm/agent/service.rs#L856-L858
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/default_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/App/provider_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn provider_context_window(&self) -> Option<u32>`

# Called by

- [default_context_window](../../../../../../functions/src/llm/agent/service/AgentService/default_context_window.md)
- [provider_context_window](../../../../../../functions/src/tui/app/App/provider_context_window.md)