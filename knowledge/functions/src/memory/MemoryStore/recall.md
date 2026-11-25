---
type: Rust Method
title: recall
resource: src/memory/mod.rs#L348-L356
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/benches/memory_recall/bench_keyword_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/memory_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_acceptance_test/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/first_request_block
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_semantic_test/recall_in_window
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_includes_own_project_and_global_only
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_records_what_it_sent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recall_is_none_when_nothing_is_stored
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn recall(&self, request: &RecallRequest<'_>) -> Option<String>`

# Calls

- [try_recall](../../../../functions/src/memory/MemoryStore/try_recall.md)

# Called by

- [bench_keyword_recall](../../../../functions/benches/memory_recall/bench_keyword_recall.md)
- [memory_block](../../../../functions/src/llm/agent/service/AgentService/memory_block.md)
- [recall](../../../../functions/tests/memory_acceptance_test/recall.md)
- [run](../../../../functions/tests/memory_benchmark_test/run.md)
- [recall](../../../../functions/tests/memory_extraction_test/recall.md)
- [a_session_end_run_writes_an_episode_that_starts_the_next_session](../../../../functions/tests/memory_extraction_test/a_session_end_run_writes_an_episode_that_starts_the_next_session.md)
- [first_request_block](../../../../functions/tests/memory_extraction_test/first_request_block.md)
- [a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget](../../../../functions/tests/memory_paths_test/a_mentioned_path_brings_its_memory_in_even_under_a_tight_budget.md)
- [an_absolute_path_in_the_message_brings_its_memory_in](../../../../functions/tests/memory_paths_test/an_absolute_path_in_the_message_brings_its_memory_in.md)
- [recall_in_window](../../../../functions/tests/memory_semantic_test/recall_in_window.md)
- [recall_includes_own_project_and_global_only](../../../../functions/tests/memory_store_test/recall_includes_own_project_and_global_only.md)
- [recent_work_is_added_only_on_the_first_request](../../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)
- [recall_records_what_it_sent](../../../../functions/tests/memory_store_test/recall_records_what_it_sent.md)
- [recall_is_none_when_nothing_is_stored](../../../../functions/tests/memory_store_test/recall_is_none_when_nothing_is_stored.md)