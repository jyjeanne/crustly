---
type: Rust Method
title: save_to_store
resource: src/llm/tools/save_memory.rs#L49-L153
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/models/PlanTaskStatus/parse
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn save_to_store( &self, store: &MemoryStore, input: SaveMemoryInput, context: &ToolExecutionContext, ) -> Result<ToolResult>`

# Calls

- [identify](../../../../../../functions/src/memory/project/identify.md)
- [register_project](../../../../../../functions/src/memory/MemoryStore/register_project.md)
- [error](../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [parse](../../../../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [find](../../../../../../functions/src/memory/MemoryStore/find.md)
- [contains](../../../../../../functions/src/memory/vector_index/VectorIndex/contains.md)
- [save](../../../../../../functions/src/memory/MemoryStore/save.md)
- [add_paths](../../../../../../functions/src/memory/MemoryStore/add_paths.md)
- [success](../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/tool/execute.md)