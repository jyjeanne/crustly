---
type: Rust Function
title: an_unusable_answer_is_retried_then_skipped
resource: tests/memory_extraction_test.rs#L308-L339
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/enabled_config
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/session_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_unusable_answer_is_retried_then_skipped()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [enabled_config](../../../functions/tests/memory_extraction_test/enabled_config.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [session_with](../../../functions/tests/memory_extraction_test/session_with.md)
- [chat](../../../functions/tests/memory_extraction_test/chat.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [extract](../../../functions/src/memory/extraction/MemoryStore/extract.md)