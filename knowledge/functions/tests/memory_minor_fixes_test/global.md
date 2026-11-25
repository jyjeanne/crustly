---
type: Rust Function
title: global
resource: tests/memory_minor_fixes_test.rs#L26-L37
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/tests/memory_minor_fixes_test/a_duplicate_still_supersedes_the_memory_it_replaces
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn global(content: &str, replaces: Option<Uuid>) -> NewMemory`

# Called by

- [a_duplicate_still_supersedes_the_memory_it_replaces](../../../functions/tests/memory_minor_fixes_test/a_duplicate_still_supersedes_the_memory_it_replaces.md)