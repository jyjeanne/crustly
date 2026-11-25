---
type: Rust Method
title: run
resource: src/llm/agent/service/run.rs#L82-L243
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/run/AgentRun/check_cancelled
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/before_round
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/AgentRun/finish
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/AgentRun/outcome
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/make_room
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/call_model
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/record_calibration
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/add_usage
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/log_blocks
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/resolve_name
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/on_final_answer
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/context/AgentContext/add_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/provider/types/Message/user
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/registry/ToolRegistry/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/middleware/Pipeline/on_tool_calls
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/middleware/Pipeline/after_tools
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn run(mut self) -> Result<RunOutcome>`

# Calls

- [check_cancelled](../../../../../../../functions/src/llm/agent/service/run/AgentRun/check_cancelled.md)
- [before_round](../../../../../../../functions/src/llm/agent/middleware/Pipeline/before_round.md)
- [finish](../../../../../../../functions/src/llm/agent/service/run/AgentRun/finish.md)
- [outcome](../../../../../../../functions/src/llm/agent/service/run/AgentRun/outcome.md)
- [make_room](../../../../../../../functions/src/llm/agent/service/AgentService/make_room.md)
- [call_model](../../../../../../../functions/src/llm/agent/service/run/AgentRun/call_model.md)
- [record_calibration](../../../../../../../functions/src/llm/agent/service/AgentService/record_calibration.md)
- [add_usage](../../../../../../../functions/src/llm/agent/service/run/AgentRun/add_usage.md)
- [log_blocks](../../../../../../../functions/src/llm/agent/service/run/log_blocks.md)
- [resolve_name](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/resolve_name.md)
- [on_final_answer](../../../../../../../functions/src/llm/agent/middleware/Pipeline/on_final_answer.md)
- [add_message](../../../../../../../functions/src/llm/agent/context/AgentContext/add_message.md)
- [user](../../../../../../../functions/src/llm/provider/types/Message/user.md)
- [get](../../../../../../../functions/src/llm/tools/registry/ToolRegistry/get.md)
- [on_tool_calls](../../../../../../../functions/src/llm/agent/middleware/Pipeline/on_tool_calls.md)
- [execute_tool_round](../../../../../../../functions/src/llm/agent/service/tool_round/AgentService/execute_tool_round.md)
- [path_memory_text](../../../../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [after_tools](../../../../../../../functions/src/llm/agent/middleware/Pipeline/after_tools.md)

# Called by

- [send_message_with_tools_inner](../../../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)