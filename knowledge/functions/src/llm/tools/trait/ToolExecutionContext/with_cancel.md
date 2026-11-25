---
type: Rust Method
title: with_cancel
resource: src/llm/tools/trait.rs#L152-L155
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn with_cancel(mut self, cancel: CancellationToken) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)