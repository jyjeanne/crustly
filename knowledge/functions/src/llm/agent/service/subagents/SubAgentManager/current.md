---
type: Rust Method
title: current
resource: src/llm/agent/service/subagents.rs#L175-L177
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/list
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn current(&self, id: Uuid) -> Option<SubAgentReport>`

# Calls

- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)

# Called by

- [report](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)
- [list](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/list.md)
- [execute](../../../../../../../functions/src/llm/agent/service/subagents/execute.md)