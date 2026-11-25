---
type: Rust Method
title: run
resource: src/llm/agent/service/subagents.rs#L178-L187
generated:
  by: okf-rs/0.7.0
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
  called_by:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn run( self: &Arc<Self>, template: &ChildTemplate, task: SubAgentTask, cancel: CancellationToken, ) -> SubAgentReport`

# Calls

- [profile_name](../../../../../../../functions/src/llm/agent/service/subagents/profile_name.md)
- [register](../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/register.md)
- [execute](../../../../../../../functions/src/llm/agent/service/subagents/execute.md)

# Called by

- [run](../../../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/run.md)