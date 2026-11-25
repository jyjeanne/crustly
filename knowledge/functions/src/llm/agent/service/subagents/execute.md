---
type: Rust Function
title: execute
resource: src/llm/agent/service/subagents.rs#L272-L361
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/current
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/subagents/write_report
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/ChildTemplate/build
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_cancellable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/shorten
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/write_answer
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/subagents/SubAgentManager/start
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute( manager: Arc<SubAgentManager>, template: ChildTemplate, task: SubAgentTask, profile_name: String, cancel: CancellationToken, ) -> SubAgentReport`

# Calls

- [current](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/current.md)
- [update](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/update.md)
- [write_report](../../../../../../functions/src/llm/agent/service/subagents/write_report.md)
- [build](../../../../../../functions/src/llm/agent/service/subagents/ChildTemplate/build.md)
- [new](../../../../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../../../../functions/src/services/session/SessionService/create_session.md)
- [send_message_with_tools_cancellable](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_cancellable.md)
- [shorten](../../../../../../functions/src/llm/agent/service/subagents/shorten.md)
- [write_answer](../../../../../../functions/src/llm/agent/service/subagents/write_answer.md)

# Called by

- [run](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/run.md)
- [start](../../../../../../functions/src/llm/agent/service/subagents/SubAgentManager/start.md)