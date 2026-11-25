---
type: Rust Function
title: launcher_for
resource: src/llm/agent/service.rs#L3748-L3750
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/child_template
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn launcher_for(service: &AgentService) -> subagents::AgentServiceLauncher`

# Calls

- [new](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/new.md)
- [child_template](../../../../../functions/src/llm/agent/service/AgentService/child_template.md)

# Called by

- [a_background_sub_agent_is_collected_with_its_report](../../../../../functions/src/llm/agent/service/a_background_sub_agent_is_collected_with_its_report.md)
- [an_unknown_profile_fails_with_the_available_ones](../../../../../functions/src/llm/agent/service/an_unknown_profile_fails_with_the_available_ones.md)
- [sub_agents_run_one_at_a_time_by_default](../../../../../functions/src/llm/agent/service/sub_agents_run_one_at_a_time_by_default.md)
- [a_running_sub_agent_can_be_cancelled](../../../../../functions/src/llm/agent/service/a_running_sub_agent_can_be_cancelled.md)