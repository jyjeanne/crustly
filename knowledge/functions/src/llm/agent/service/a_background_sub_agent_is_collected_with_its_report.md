---
type: Rust Function
title: a_background_sub_agent_is_collected_with_its_report
resource: src/llm/agent/service.rs#L3739-L3765
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
  - target: functions/src/llm/agent/service/task
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn a_background_sub_agent_is_collected_with_its_report()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_working_directory](../../../../../functions/src/llm/agent/service/AgentService/with_working_directory.md)
- [launcher_for](../../../../../functions/src/llm/agent/service/launcher_for.md)
- [task](../../../../../functions/src/llm/agent/service/task.md)
- [start](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/start.md)
- [report](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/report.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [list](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/list.md)
- [cancel](../../../../../functions/src/llm/agent/service/subagents/AgentServiceLauncher/subagentlauncher/cancel.md)