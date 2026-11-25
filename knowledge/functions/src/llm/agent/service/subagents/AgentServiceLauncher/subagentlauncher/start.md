---
type: Rust Method
title: start
resource: src/llm/agent/service/subagents.rs#L440-L447
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn start( &self, task: SubAgentTask, cancel: CancellationToken, ) -> std::result::Result<(), String>`

# Calls

- [start](../../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)

# Called by

- [a_background_sub_agent_is_collected_with_its_report](../../../../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [sub_agents_run_one_at_a_time_by_default](../../../../../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)