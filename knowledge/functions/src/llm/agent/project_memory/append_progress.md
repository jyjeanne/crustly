---
type: Rust Function
title: append_progress
resource: src/llm/agent/project_memory.rs#L169-L201
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/tool_round/ToolRound/push
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/project_memory/notes_point_at_the_progress_log_when_there_is_one
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/the_progress_log_grows_and_drops_its_oldest_entries
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/project_memory/log_entry
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/log_progress
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn append_progress(dir: &Path, entry: &ProgressEntry) -> std::io::Result<()>`

# Calls

- [push](../../../../../functions/src/llm/agent/service/tool_round/ToolRound/push.md)
- [find](../../../../../functions/src/memory/MemoryStore/find.md)
- [write](../../../../../functions/src/memory/project/write.md)

# Called by

- [notes_point_at_the_progress_log_when_there_is_one](../../../../../functions/src/llm/agent/project_memory/notes_point_at_the_progress_log_when_there_is_one.md)
- [the_progress_log_grows_and_drops_its_oldest_entries](../../../../../functions/src/llm/agent/project_memory/the_progress_log_grows_and_drops_its_oldest_entries.md)
- [log_entry](../../../../../functions/src/llm/agent/project_memory/log_entry.md)
- [log_progress](../../../../../functions/src/llm/agent/service/AgentService/log_progress.md)
- [recent_work_is_added_only_on_the_first_request](../../../../../functions/tests/memory_store_test/recent_work_is_added_only_on_the_first_request.md)