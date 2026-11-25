---
type: Rust Method
title: pending
resource: src/memory/extraction.rs#L327-L329
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/pending_for
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/reload_memory_rows
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn pending(&self, project: &ProjectIdentity) -> Result<Vec<super::model::Memory>>`

# Calls

- [pending_for](../../../../../functions/src/db/repository/memory/MemoryRepository/pending_for.md)

# Called by

- [cmd_memory](../../../../../functions/src/cli/cmd_memory.md)
- [cancelling_during_an_approval_prompt_ends_the_request](../../../../../functions/src/llm/agent/service/cancelling_during_an_approval_prompt_ends_the_request.md)
- [reload_memory_rows](../../../../../functions/src/tui/app/App/reload_memory_rows.md)
- [review_holds_memories_until_approved_and_rejection_deletes_them](../../../../../functions/tests/memory_extraction_test/review_holds_memories_until_approved_and_rejection_deletes_them.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)