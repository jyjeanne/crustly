---
type: Rust Function
title: first_request_block
resource: tests/memory_extraction_test.rs#L686-L702
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/recall
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn first_request_block( store: &MemoryStore, project: &ProjectIdentity, window: u32, ) -> String`

# Calls

- [recall](../../../functions/src/memory/MemoryStore/recall.md)

# Called by

- [an_episode_in_recent_work_is_not_listed_twice](../../../functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)