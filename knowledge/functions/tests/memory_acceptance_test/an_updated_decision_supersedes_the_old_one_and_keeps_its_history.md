---
type: Rust Function
title: an_updated_decision_supersedes_the_old_one_and_keeps_its_history
resource: tests/memory_acceptance_test.rs#L148-L202
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_updated_decision_supersedes_the_old_one_and_keeps_its_history()`

# Calls

- [open](../../../functions/tests/memory_acceptance_test/open.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)