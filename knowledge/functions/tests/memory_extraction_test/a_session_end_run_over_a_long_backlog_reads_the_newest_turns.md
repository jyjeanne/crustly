---
type: Rust Function
title: a_session_end_run_over_a_long_backlog_reads_the_newest_turns
resource: tests/memory_extraction_test.rs#L652-L667
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/memory_extraction_test/Scripted/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
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
  - target: functions/tests/memory_extraction_test/chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/session_with
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/Scripted/last_prompt
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn a_session_end_run_over_a_long_backlog_reads_the_newest_turns()`

# Calls

- [new](../../../functions/tests/memory_extraction_test/Scripted/new.md)
- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [enabled_config](../../../functions/tests/memory_extraction_test/enabled_config.md)
- [with_extraction](../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [chat](../../../functions/tests/memory_extraction_test/chat.md)
- [session_with](../../../functions/tests/memory_extraction_test/session_with.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [extract](../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [last_prompt](../../../functions/tests/memory_extraction_test/Scripted/last_prompt.md)