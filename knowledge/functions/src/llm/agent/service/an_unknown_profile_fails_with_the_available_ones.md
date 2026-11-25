---
type: Rust Function
title: an_unknown_profile_fails_with_the_available_ones
resource: src/llm/agent/service.rs#L3782-L3799
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
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/launcher_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/run
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_unknown_profile_fails_with_the_available_ones()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_working_directory](../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [launcher_for](../../../../../functions/src/llm/agent/service/launcher_for.md)
- [run](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/run.md)
- [task](../../../../../functions/src/llm/agent/service/task.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)