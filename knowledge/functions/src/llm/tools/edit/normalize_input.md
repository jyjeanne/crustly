---
type: Rust Function
title: normalize_input
resource: src/llm/tools/edit.rs#L93-L105
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/edit/EditTool/tool/validate_input
  - functions/src/llm/tools/edit/EditTool/tool/execute
---

# Signature

`fn normalize_input(mut input: Value) -> Value`

# Called by

- [validate_input](../../../../../functions/src/llm/tools/edit/EditTool/tool/validate_input.md)
- [execute](../../../../../functions/src/llm/tools/edit/EditTool/tool/execute.md)