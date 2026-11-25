---
type: Rust Function
title: strip_verbatim_prefix
resource: src/llm/tools/sandbox.rs#L196-L224
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/tui/events/EventHandler/next
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/plan/PlanTask/skip
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/sandbox/PathBoundaryRule/check
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_verbatim_prefix(path: &Path) -> PathBuf`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [skip](../../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)