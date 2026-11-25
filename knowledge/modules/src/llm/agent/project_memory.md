---
type: Rust Module
title: project_memory
resource: src/llm/agent/project_memory.rs#L1-L434
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/super-context-token-count
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super-middleware-toolcallrecord
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-value
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-path-path
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/serde-json-json
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [notes_budget](../../../../functions/src/llm/agent/project_memory/notes_budget.md)
- [load_project_notes](../../../../functions/src/llm/agent/project_memory/load_project_notes.md)
- [take_tokens](../../../../functions/src/llm/agent/project_memory/take_tokens.md)
- [summarize_tool_calls](../../../../functions/src/llm/agent/project_memory/summarize_tool_calls.md)
- [call_target](../../../../functions/src/llm/agent/project_memory/call_target.md)
- [ProgressEntry](../../../../classes/src/llm/agent/project_memory/ProgressEntry.md)
- [append_progress](../../../../functions/src/llm/agent/project_memory/append_progress.md)
- [first_line](../../../../functions/src/llm/agent/project_memory/first_line.md)
- [recent_work](../../../../functions/src/llm/agent/project_memory/recent_work.md)
- [record](../../../../functions/src/llm/agent/project_memory/record.md)
- [notes_come_from_the_files_present_in_order](../../../../functions/src/llm/agent/project_memory/notes_come_from_the_files_present_in_order.md)
- [notes_are_truncated_to_the_budget_at_line_ends](../../../../functions/src/llm/agent/project_memory/notes_are_truncated_to_the_budget_at_line_ends.md)
- [notes_point_at_the_progress_log_when_there_is_one](../../../../functions/src/llm/agent/project_memory/notes_point_at_the_progress_log_when_there_is_one.md)
- [notes_budget_is_an_eighth_of_the_window_up_to_2000](../../../../functions/src/llm/agent/project_memory/notes_budget_is_an_eighth_of_the_window_up_to_2000.md)
- [tool_calls_are_summarized_with_their_targets](../../../../functions/src/llm/agent/project_memory/tool_calls_are_summarized_with_their_targets.md)
- [long_requests_keep_the_first_calls_and_count_the_rest](../../../../functions/src/llm/agent/project_memory/long_requests_keep_the_first_calls_and_count_the_rest.md)
- [the_progress_log_grows_and_drops_its_oldest_entries](../../../../functions/src/llm/agent/project_memory/the_progress_log_grows_and_drops_its_oldest_entries.md)
- [log_entry](../../../../functions/src/llm/agent/project_memory/log_entry.md)
- [recent_work_gives_the_last_entries_oldest_first](../../../../functions/src/llm/agent/project_memory/recent_work_gives_the_last_entries_oldest_first.md)
- [recent_work_is_empty_without_a_log](../../../../functions/src/llm/agent/project_memory/recent_work_is_empty_without_a_log.md)
- [recent_work_drops_the_oldest_entries_first_past_the_cap](../../../../functions/src/llm/agent/project_memory/recent_work_drops_the_oldest_entries_first_past_the_cap.md)

# Imports

- `super::context::token_count`
- `super::middleware::ToolCallRecord`
- `serde_json::Value`
- `std::path::Path`
- `super::*`
- `serde_json::json`

# Member of

- [crustly](../../../../packages/crustly.md)