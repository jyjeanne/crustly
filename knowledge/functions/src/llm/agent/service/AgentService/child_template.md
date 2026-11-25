---
type: Rust Method
title: child_template
resource: src/llm/agent/service.rs#L1271-L1290
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/launcher_for
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn child_template(&self) -> subagents::ChildTemplate`

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [launcher_for](../../../../../../functions/src/llm/agent/service/launcher_for.md)