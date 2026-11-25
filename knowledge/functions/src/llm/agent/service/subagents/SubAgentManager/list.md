---
type: Rust Method
title: list
resource: src/llm/agent/service/subagents.rs#L248-L254
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/current
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn list(&self) -> Vec<SubAgentReport>`

# Calls

- [current](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/current.md)

# Called by

- [list](../../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/list.md)