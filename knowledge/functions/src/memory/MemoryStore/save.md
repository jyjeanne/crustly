---
type: Rust Method
title: save
resource: src/memory/mod.rs#L219-L221
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/save_with
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/cli/reject_refuses_a_memory_that_is_not_pending
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/forget_memory/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/search_memory/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/consolidate/MemoryStore/lesson_from_episodes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/transfer/import
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/transfer/import_from_notes
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/tui/app/memory_screen_app
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_acceptance_test/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_benchmark_test/run
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
  - target: functions/tests/memory_extraction_test/save_episode
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_paths_test/save_in_project
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_semantic_test/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/tests/memory_transfer_test/save
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn save(&self, new: NewMemory) -> Result<SaveOutcome>`

# Calls

- [save_with](../../../../functions/src/memory/MemoryStore/save_with.md)

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [reject_refuses_a_memory_that_is_not_pending](../../../../functions/src/cli/reject_refuses_a_memory_that_is_not_pending.md)
- [recall_block_reaches_the_model_but_not_the_database](../../../../functions/src/llm/agent/service/recall_block_reaches_the_model_but_not_the_database.md)
- [a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory](../../../../functions/src/llm/agent/service/a_refused_forget_deletes_nothing_and_the_prompt_shows_the_memory.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [save](../../../../functions/src/llm/tools/forget_memory/save.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)
- [save](../../../../functions/src/llm/tools/search_memory/save.md)
- [lesson_from_episodes](../../../../functions/src/memory/consolidate/MemoryStore/lesson_from_episodes.md)
- [import](../../../../functions/src/memory/transfer/import.md)
- [import_from_notes](../../../../functions/src/memory/transfer/import_from_notes.md)
- [memory_screen_app](../../../../functions/src/tui/app/memory_screen_app.md)
- [save](../../../../functions/tests/memory_acceptance_test/save.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence](../../../../functions/tests/memory_extraction_test/an_extracted_memory_cannot_replace_a_user_memory_without_user_evidence.md)
- [an_approved_correction_supersedes_the_memory_it_corrects](../../../../functions/tests/memory_extraction_test/an_approved_correction_supersedes_the_memory_it_corrects.md)
- [the_user_saying_a_pending_memory_approves_it](../../../../functions/tests/memory_extraction_test/the_user_saying_a_pending_memory_approves_it.md)
- [save_episode](../../../../functions/tests/memory_extraction_test/save_episode.md)
- [save_in_project](../../../../functions/tests/memory_paths_test/save_in_project.md)
- [save](../../../../functions/tests/memory_semantic_test/save.md)
- [save](../../../../functions/tests/memory_transfer_test/save.md)