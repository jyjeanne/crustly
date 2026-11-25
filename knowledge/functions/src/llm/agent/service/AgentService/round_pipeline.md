---
type: Rust Method
title: round_pipeline
resource: src/llm/agent/service.rs#L1025-L1046
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/middleware/Pipeline/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/Pipeline/with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/LoopGuard/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/tool_profile/ToolSelection/exposes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/has_tool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Verification/detect_command
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Verification/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn round_pipeline(&self, selection: &ToolSelection, read_only: bool) -> Pipeline`

# Calls

- [new](../../../../../../functions/src/llm/agent/middleware/Pipeline/new.md)
- [with](../../../../../../functions/src/llm/agent/middleware/Pipeline/with.md)
- [new](../../../../../../functions/src/llm/agent/middleware/LoopGuard/new.md)
- [exposes](../../../../../../functions/src/llm/agent/tool_profile/ToolSelection/exposes.md)
- [has_tool](../../../../../../functions/src/llm/tools/registry/ToolRegistry/has_tool.md)
- [detect_command](../../../../../../functions/src/llm/agent/middleware/Verification/detect_command.md)
- [new](../../../../../../functions/src/llm/agent/middleware/Verification/new.md)

# Called by

- [send_message_with_tools_inner](../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)