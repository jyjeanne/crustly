---
type: Rust Function
title: a_running_sub_agent_can_be_cancelled
resource: src/llm/agent/service.rs#L3896-L3921
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn a_running_sub_agent_can_be_cancelled()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_working_directory](../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [launcher_for](../../../../../functions/src/llm/agent/service/launcher_for.md)
- [task](../../../../../functions/src/llm/agent/service/task.md)
- [start](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start.md)
- [wait_for_status](../../../../../functions/src/llm/agent/service/wait_for_status.md)
- [report](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)
- [cancel](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel.md)