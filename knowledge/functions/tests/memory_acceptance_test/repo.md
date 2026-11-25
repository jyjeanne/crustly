---
type: Rust Function
title: repo
resource: tests/memory_acceptance_test.rs#L24-L32
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/id_for_root
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn repo(name: &str) -> ProjectIdentity`

# Calls

- [id_for_root](../../../functions/src/memory/project/id_for_root.md)