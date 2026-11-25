---
type: Rust Method
title: as_str
resource: src/memory/model.rs#L32-L41
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/memory_line
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/memory/MemoryRepository/insert
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/describe
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/result_line
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/recall/line
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub fn as_str(self) -> &'static str`

# Called by

- [memory_line](../../../../../functions/src/cli/memory_line.md)
- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [insert](../../../../../functions/src/db/repository/memory/MemoryRepository/insert.md)
- [describe](../../../../../functions/src/llm/tools/forget_memory/describe.md)
- [result_line](../../../../../functions/src/llm/tools/search_memory/result_line.md)
- [save_with](../../../../../functions/src/memory/MemoryStore/save_with.md)
- [line](../../../../../functions/src/memory/recall/line.md)