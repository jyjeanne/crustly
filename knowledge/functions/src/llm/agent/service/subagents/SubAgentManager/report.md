---
type: Rust Method
title: report
resource: src/llm/agent/service/subagents.rs#L212-L234
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/current
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/SubAgentStatus/is_finished
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn report(&self, id: Uuid, wait: bool) -> Option<SubAgentReport>`

# Calls

- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)
- [current](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/current.md)
- [is_finished](../../../../../../../functions/src/llm/tools/trait/SubAgentStatus/is_finished.md)
- [update](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/update.md)

# Called by

- [report](../../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)