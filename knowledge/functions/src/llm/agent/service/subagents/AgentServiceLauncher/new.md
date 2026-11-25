---
type: Rust Method
title: new
resource: src/llm/agent/service/subagents.rs#L419-L421
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/launcher_for
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn new(manager: Arc<SubAgentManager>, template: ChildTemplate) -> Self`

# Called by

- [send_message_with_tools_inner](../../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [launcher_for](../../../../../../../functions/src/llm/agent/service/launcher_for.md)