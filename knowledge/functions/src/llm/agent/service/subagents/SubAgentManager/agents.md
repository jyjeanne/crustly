---
type: Rust Method
title: agents
resource: src/llm/agent/service/subagents.rs#L139-L141
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/register
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/update
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/current
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/start
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/cancel
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn agents(&self) -> std::sync::MutexGuard<'_, HashMap<Uuid, Slot>>`

# Called by

- [register](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/register.md)
- [update](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/update.md)
- [current](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/current.md)
- [start](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)
- [report](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)
- [cancel](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/cancel.md)