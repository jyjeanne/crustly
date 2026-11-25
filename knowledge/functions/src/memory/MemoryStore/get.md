---
type: Rust Method
title: get
resource: src/memory/mod.rs#L661-L663
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn get(&self, id: Uuid) -> Result<Option<Memory>>`

# Calls

- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)

# Called by

- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one](../../../../functions/tests/memory_code_knowledge_test/a_concept_gone_from_a_newer_bundle_is_flagged_and_not_from_an_older_one.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)