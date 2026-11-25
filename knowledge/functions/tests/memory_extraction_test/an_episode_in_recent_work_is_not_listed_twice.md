---
type: Rust Function
title: an_episode_in_recent_work_is_not_listed_twice
resource: tests/memory_extraction_test.rs#L705-L723
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/config/MemoryConfig/default/default
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/save_episode
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/first_request_block
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_episode_in_recent_work_is_not_listed_twice()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_episode](../../../functions/tests/memory_extraction_test/save_episode.md)
- [first_request_block](../../../functions/tests/memory_extraction_test/first_request_block.md)