---
type: Rust Function
title: save_episode
resource: tests/memory_extraction_test.rs#L669-L684
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn save_episode(store: &MemoryStore, project: &ProjectIdentity, content: &str)`

# Calls

- [save](../../../functions/src/memory/MemoryStore/save.md)

# Called by

- [an_episode_in_recent_work_is_not_listed_twice](../../../functions/tests/memory_extraction_test/an_episode_in_recent_work_is_not_listed_twice.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)