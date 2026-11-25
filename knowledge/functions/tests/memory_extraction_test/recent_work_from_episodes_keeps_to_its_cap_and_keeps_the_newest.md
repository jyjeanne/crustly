---
type: Rust Function
title: recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
resource: tests/memory_extraction_test.rs#L726-L755
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
  - target: functions/src/llm/agent/context/token_count
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest()`

# Calls

- [new](../../../functions/src/memory/MemoryStore/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [default](../../../functions/src/config/MemoryConfig/default/default.md)
- [register_project](../../../functions/src/memory/MemoryStore/register_project.md)
- [save_episode](../../../functions/tests/memory_extraction_test/save_episode.md)
- [first_request_block](../../../functions/tests/memory_extraction_test/first_request_block.md)
- [token_count](../../../functions/src/llm/agent/context/token_count.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)