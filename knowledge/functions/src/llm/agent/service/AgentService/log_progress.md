---
type: Rust Method
title: log_progress
resource: src/llm/agent/service.rs#L1038-L1064
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/is_file_edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/append_progress
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn log_progress( &self, request: &str, tools: Option<&str>, answer: &str, outcome: &run::RunOutcome, read_only: bool, )`

# Calls

- [is_file_edit](../../../../../../functions/src/llm/agent/middleware/is_file_edit.md)
- [append_progress](../../../../../../functions/src/llm/agent/project_memory/append_progress.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)