---
type: Rust Method
title: provider_context_window
resource: src/tui/app.rs#L514-L516
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/provider_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/render/render_model_info
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn provider_context_window(&self) -> Option<u32>`

# Calls

- [provider_context_window](../../../../../functions/src/llm/agent/service/AgentService/provider_context_window.md)

# Called by

- [render_model_info](../../../../../functions/src/tui/render/render_model_info.md)