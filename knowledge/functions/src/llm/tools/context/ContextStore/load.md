---
type: Rust Method
title: load
resource: src/llm/tools/context.rs#L55-L66
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/context/ContextStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/context/ContextTool/tool/execute
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn load(path: &Path, session_id: &str) -> Result<Self>`

# Calls

- [new](../../../../../../functions/src/llm/tools/context/ContextStore/new.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/context/ContextTool/tool/execute.md)