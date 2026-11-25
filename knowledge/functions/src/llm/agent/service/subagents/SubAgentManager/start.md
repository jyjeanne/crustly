---
type: Rust Method
title: start
resource: src/llm/agent/service/subagents.rs#L192-L211
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/profile_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/register
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/agents
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn start( self: &Arc<Self>, template: &ChildTemplate, task: SubAgentTask, cancel: CancellationToken, )`

# Calls

- [profile_name](../../../../../../../functions/src/llm/agent/service/subagents/profile_name.md)
- [register](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/register.md)
- [execute](../../../../../../../functions/src/llm/agent/service/subagents/execute.md)
- [agents](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/agents.md)

# Called by

- [start](../../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start.md)