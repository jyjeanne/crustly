---
type: Rust Function
title: token_count
resource: src/llm/agent/context.rs#L232-L248
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/agent/budget/tool_schema_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/AgentContext/estimate_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count_prose_reasonable
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/context/token_count_empty_string
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/load_project_notes
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/take_tokens
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/try_recall
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/recent_work
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/select
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_benchmark_test/run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_minor_fixes_test/the_budget_counts_each_lines_overhead_not_just_its_text
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn token_count(text: &str) -> u32`

# Called by

- [tool_schema_tokens](../../../../../functions/src/llm/agent/budget/tool_schema_tokens.md)
- [elide_tool_results](../../../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [estimate_tokens](../../../../../functions/src/llm/agent/context/AgentContext/estimate_tokens.md)
- [token_count_bpe_accuracy_rust_file](../../../../../functions/src/llm/agent/context/token_count_bpe_accuracy_rust_file.md)
- [token_count_prose_reasonable](../../../../../functions/src/llm/agent/context/token_count_prose_reasonable.md)
- [token_count_empty_string](../../../../../functions/src/llm/agent/context/token_count_empty_string.md)
- [load_project_notes](../../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
- [take_tokens](../../../../../functions/src/llm/agent/project_memory/take_tokens.md)
- [recent_work](../../../../../functions/src/llm/agent/project_memory/recent_work.md)
- [recent_work_drops_the_oldest_entries_first_past_the_cap](../../../../../functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap.md)
- [notes_are_truncated_to_the_budget_at_line_ends](../../../../../functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends.md)
- [path_memory_text](../../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [try_recall](../../../../../functions/src/memory/MemoryStore/try_recall.md)
- [recent_work](../../../../../functions/src/memory/MemoryStore/recent_work.md)
- [select](../../../../../functions/src/memory/recall/select.md)
- [pinned_memories_use_at_most_half_the_budget](../../../../../functions/src/memory/recall/pinned_memories_use_at_most_half_the_budget.md)
- [run](../../../../../functions/tests/memory_benchmark_test/run.md)
- [recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest](../../../../../functions/tests/memory_extraction_test/recent_work_from_episodes_keeps_to_its_cap_and_keeps_the_newest.md)
- [the_budget_counts_each_lines_overhead_not_just_its_text](../../../../../functions/tests/memory_minor_fixes_test/the_budget_counts_each_lines_overhead_not_just_its_text.md)