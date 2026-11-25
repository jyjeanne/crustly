---
type: Rust Function
title: wait_for_status
resource: src/llm/agent/service.rs#L3838-L3851
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn wait_for_status( launcher: &subagents::AgentServiceLauncher, id: Uuid, status: crate::llm::tools::SubAgentStatus, )`

# Calls

- [report](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)

# Called by

- [sub_agents_run_one_at_a_time_by_default](../../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)