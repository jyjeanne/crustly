---
type: Rust Method
title: evaluate
resource: src/llm/tools/sandbox.rs#L227-L240
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/check
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/absolute_path_outside_root_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/valid_path_inside_root_allowed
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/path_traversal_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/sandbox/symlink_outside_root_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`fn evaluate(&self, _tool_name: &str, inputs: &Value) -> PolicyDecision`

# Calls

- [check](../../../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)

# Called by

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [absolute_path_outside_root_denied](../../../../../../../functions/src/llm/tools/sandbox/absolute_path_outside_root_denied.md)
- [valid_path_inside_root_allowed](../../../../../../../functions/src/llm/tools/sandbox/valid_path_inside_root_allowed.md)
- [path_traversal_denied](../../../../../../../functions/src/llm/tools/sandbox/path_traversal_denied.md)
- [symlink_outside_root_denied](../../../../../../../functions/src/llm/tools/sandbox/symlink_outside_root_denied.md)