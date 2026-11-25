---
type: Rust Function
title: sub_agents_run_one_at_a_time_by_default
resource: src/llm/agent/service.rs#L3842-L3879
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/launcher_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/wait_for_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn sub_agents_run_one_at_a_time_by_default()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_working_directory](../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [launcher_for](../../../../../functions/src/llm/agent/service/launcher_for.md)
- [task](../../../../../functions/src/llm/agent/service/task.md)
- [start](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start.md)
- [wait_for_status](../../../../../functions/src/llm/agent/service/wait_for_status.md)
- [report](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)