---
type: Rust Method
title: default_context_window
resource: src/llm/agent/service.rs#L878-L881
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/AgentService/provider_context_window
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/exposed_tool_names
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/context_usage_preview
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn default_context_window(&self) -> u32`

# Calls

- [provider_context_window](../../../../../../functions/src/llm/agent/service/AgentService/provider_context_window.md)

# Called by

- [exposed_tool_names](../../../../../../functions/src/llm/agent/service/AgentService/exposed_tool_names.md)
- [context_usage_preview](../../../../../../functions/src/llm/agent/service/AgentService/context_usage_preview.md)