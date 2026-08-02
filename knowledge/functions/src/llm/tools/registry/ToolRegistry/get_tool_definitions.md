---
type: Rust Method
title: get_tool_definitions
resource: src/llm/tools/registry.rs#L119-L128
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
---

# Signature

`pub fn get_tool_definitions(&self) -> Vec<crate::llm::provider::Tool>`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)