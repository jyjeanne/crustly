---
type: Rust Method
title: memories_for_path
resource: src/memory/mod.rs#L774-L809
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/paths/project_relative
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/path_globs_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/paths/matches
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/agent/service/run/AgentRun/path_memory_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn memories_for_path( &self, project: &ProjectIdentity, session: Uuid, path: &str, working_dir: &Path, ) -> Result<Vec<Memory>>`

# Calls

- [project_relative](../../../../functions/src/memory/paths/project_relative.md)
- [path_globs_for](../../../../functions/src/db/repository/memory/MemoryRepository/path_globs_for.md)
- [matches](../../../../functions/src/memory/paths/matches.md)
- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)

# Called by

- [path_memory_text](../../../../functions/src/llm/agent/service/run/AgentRun/path_memory_text.md)
- [a_path_memory_is_shown_once_per_session](../../../../functions/tests/memory_paths_test/a_path_memory_is_shown_once_per_session.md)
- [tool_paths_match_when_absolute_or_relative_to_a_subdirectory](../../../../functions/tests/memory_paths_test/tool_paths_match_when_absolute_or_relative_to_a_subdirectory.md)