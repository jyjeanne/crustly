---
type: Rust Method
title: list
resource: src/llm/agent/service/subagents.rs#L455-L457
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn list(&self) -> Vec<SubAgentReport>`

# Calls

- [list](../../../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/list.md)

# Called by

- [a_background_sub_agent_is_collected_with_its_report](../../../../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)