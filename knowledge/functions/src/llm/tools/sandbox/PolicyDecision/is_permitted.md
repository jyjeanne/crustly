---
type: Rust Method
title: is_permitted
resource: src/llm/tools/sandbox.rs#L30-L32
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/sandbox/OrPolicy/permissionpolicy/evaluate
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn is_permitted(&self) -> bool`

# Called by

- [evaluate](../../../../../../functions/src/llm/tools/sandbox/OrPolicy/permissionpolicy/evaluate.md)
- [check_path](../../../../../../functions/src/llm/tools/sandbox/check_path.md)