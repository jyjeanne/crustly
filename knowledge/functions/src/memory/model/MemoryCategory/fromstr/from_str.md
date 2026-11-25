---
type: Rust Method
title: from_str
resource: src/memory/model.rs#L46-L56
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/model/ParseEnumError/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn from_str(s: &str) -> Result<Self, Self::Err>`

# Calls

- [new](../../../../../../functions/src/memory/model/ParseEnumError/new.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/search_memory/SearchMemoryTool/tool/execute.md)
- [import](../../../../../../functions/src/memory/transfer/import.md)