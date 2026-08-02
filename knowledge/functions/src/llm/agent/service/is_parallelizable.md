---
type: Rust Function
title: is_parallelizable
resource: src/llm/agent/service.rs#L227-L240
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn is_parallelizable(tool_name: &str) -> bool`

# Called by

- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)