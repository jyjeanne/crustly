---
type: Rust Method
title: count_by_status
resource: src/plan/mod.rs#L159-L161
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/plan/PlanDocument/progress_percentage
---

# Signature

`pub fn count_by_status(&self, status: TaskStatus) -> usize`

# Called by

- [progress_percentage](../../../../functions/src/plan/PlanDocument/progress_percentage.md)