---
type: Rust Method
title: update
resource: src/llm/agent/service/subagents.rs#L167-L171
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn update(&self, report: &SubAgentReport)`

# Calls

- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)

# Called by

- [report](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)
- [execute](../../../../../../../functions/src/llm/agent/service/subagents/execute.md)