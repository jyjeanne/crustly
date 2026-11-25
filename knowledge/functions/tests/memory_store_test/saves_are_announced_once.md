---
type: Rust Function
title: saves_are_announced_once
resource: tests/memory_store_test.rs#L220-L240
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/save_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/take_notices
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn saves_are_announced_once()`

# Calls

- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_request](../../../functions/tests/memory_store_test/save_request.md)
- [take_notices](../../../functions/src/memory/MemoryStore/take_notices.md)