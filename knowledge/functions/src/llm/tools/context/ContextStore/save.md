---
type: Rust Method
title: save
resource: src/llm/tools/context.rs#L68-L79
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/context/ContextTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn save(&self, path: &Path) -> Result<()>`

# Calls

- [write](../../../../../../functions/src/memory/project/write.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)