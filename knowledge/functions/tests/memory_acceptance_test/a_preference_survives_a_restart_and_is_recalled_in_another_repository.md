---
type: Rust Function
title: a_preference_survives_a_restart_and_is_recalled_in_another_repository
resource: tests/memory_acceptance_test.rs#L76-L108
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`async fn a_preference_survives_a_restart_and_is_recalled_in_another_repository()`

# Calls

- [open](../../../functions/tests/memory_acceptance_test/open.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)