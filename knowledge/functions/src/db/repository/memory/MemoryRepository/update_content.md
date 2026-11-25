---
type: Rust Method
title: update_content
resource: src/db/repository/memory.rs#L641-L666
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/record_event
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/MemoryStore/edit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/fts_search_finds_the_current_words_only
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_content( &self, id: Uuid, content: &str, content_hash: &str, actor: &str, now: DateTime<Utc>, ) -> Result<()>`

# Calls

- [record_event](../../../../../../functions/src/db/repository/memory/record_event.md)

# Called by

- [edit](../../../../../../functions/src/memory/MemoryStore/edit.md)
- [fts_search_finds_the_current_words_only](../../../../../../functions/tests/memory_repository_test/fts_search_finds_the_current_words_only.md)