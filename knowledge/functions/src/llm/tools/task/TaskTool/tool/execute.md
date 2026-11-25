---
type: Rust Method
title: execute
resource: src/llm/tools/task.rs#L388-L713
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/task/FileLock/acquire
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskStore/load
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/task/FileLock/release
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/parse_status
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/parse_priority
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/task/TaskStore/with_lock
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/remove
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/MessageLineCache/retain
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [acquire](../../../../../../../functions/src/llm/tools/task/FileLock/acquire.md)
- [load](../../../../../../../functions/src/llm/tools/task/TaskStore/load.md)
- [release](../../../../../../../functions/src/llm/tools/task/FileLock/release.md)
- [parse_status](../../../../../../../functions/src/llm/tools/task/parse_status.md)
- [parse_priority](../../../../../../../functions/src/llm/tools/task/parse_priority.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [push](../../../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [with_lock](../../../../../../../functions/src/llm/tools/task/TaskStore/with_lock.md)
- [contains](../../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [remove](../../../../../../../functions/src/memory/vector_index/VectorIndex/remove.md)
- [retain](../../../../../../../functions/src/tui/render/MessageLineCache/retain.md)