---
type: Rust Method
title: cancel
resource: src/llm/agent/service/subagents.rs#L237-L245
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/SubAgentStatus/is_finished
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn cancel(&self, id: Uuid) -> bool`

# Calls

- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)
- [is_finished](../../../../../../../functions/src/llm/tools/trait/SubAgentStatus/is_finished.md)

# Called by

- [cancel](../../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel.md)