---
type: Rust Method
title: register
resource: src/llm/agent/service/subagents.rs#L143-L165
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn register(&self, task: &SubAgentTask, profile: &str, cancel: CancellationToken)`

# Calls

- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/run.md)
- [start](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)