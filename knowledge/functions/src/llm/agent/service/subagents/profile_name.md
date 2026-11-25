---
type: Rust Function
title: profile_name
resource: src/llm/agent/service/subagents.rs#L261-L268
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/start
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn profile_name(task: &SubAgentTask) -> String`

# Called by

- [run](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/run.md)
- [start](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)