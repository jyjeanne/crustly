---
type: Rust Method
title: report
resource: src/llm/agent/service/subagents.rs#L449-L451
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/wait_for_status
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn report(&self, id: Uuid, wait: bool) -> Option<SubAgentReport>`

# Calls

- [report](../../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/report.md)

# Called by

- [a_background_sub_agent_is_collected_with_its_report](../../../../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [wait_for_status](../../../../../../../../functions/src/llm/agent/service/wait_for_status.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)
- [sub_agents_run_one_at_a_time_by_default](../../../../../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)