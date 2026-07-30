---
type: Rust Function
title: has_mutating_capability
resource: src/llm/agent/service.rs#L48-L57
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`fn has_mutating_capability(caps: &[ToolCapability]) -> bool`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)