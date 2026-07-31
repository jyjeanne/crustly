---
type: Rust Function
title: strip_verbatim_prefix
resource: src/llm/tools/sandbox.rs#L196-L224
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/tui/events/EventHandler/next
  - functions/src/plan/PlanTask/skip
  called_by:
  - functions/src/llm/tools/sandbox/PathBoundaryRule/check
---

# Signature

`fn strip_verbatim_prefix(path: &Path) -> PathBuf`

# Calls

- [next](../../../../../functions/src/tui/events/EventHandler/next.md)
- [skip](../../../../../functions/src/plan/PlanTask/skip.md)

# Called by

- [check](../../../../../functions/src/llm/tools/sandbox/PathBoundaryRule/check.md)