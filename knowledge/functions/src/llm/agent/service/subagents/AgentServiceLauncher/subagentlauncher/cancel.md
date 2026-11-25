---
type: Rust Method
title: cancel
resource: src/llm/agent/service/subagents.rs#L451-L453
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/cancel
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn cancel(&self, id: Uuid) -> bool`

# Calls

- [cancel](../../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/cancel.md)

# Called by

- [a_background_sub_agent_is_collected_with_its_report](../../../../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)