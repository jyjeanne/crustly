---
type: Rust Method
title: upsert_project
resource: src/db/repository/memory.rs#L54-L76
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/memory/extraction/MemoryStore/extract
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/register_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/relink
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/repo_with_projects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/repo
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/setup
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn upsert_project( &self, project: &ProjectIdentity, now: DateTime<Utc>, ) -> Result<()>`

# Calls

- [context](../../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [extract](../../../../../../functions/src/memory/extraction/MemoryStore/extract.md)
- [register_project](../../../../../../functions/src/memory/MemoryStore/register_project.md)
- [relink](../../../../../../functions/src/memory/MemoryStore/relink.md)
- [repo_with_projects](../../../../../../functions/tests/memory_repository_test/repo_with_projects.md)
- [relink_moves_memories_to_the_new_project](../../../../../../functions/tests/memory_repository_test/relink_moves_memories_to_the_new_project.md)
- [repo](../../../../../../functions/tests/memory_review_fixes_test/repo.md)
- [forget_is_refused_in_plan_mode_and_deletes_nothing](../../../../../../functions/tests/memory_review_fixes_test/forget_is_refused_in_plan_mode_and_deletes_nothing.md)
- [setup](../../../../../../functions/tests/memory_vectors_test/setup.md)
- [vectors_of_another_projects_are_never_returned](../../../../../../functions/tests/memory_vectors_test/vectors_of_another_projects_are_never_returned.md)