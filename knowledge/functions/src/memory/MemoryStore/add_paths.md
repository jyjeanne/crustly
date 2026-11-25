---
type: Rust Method
title: add_paths
resource: src/memory/mod.rs#L739-L766
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/repository/memory/MemoryRepository/get
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/paths/validate_glob
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/paths_of
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/memory/MemoryRepository/add_paths
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/changed
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/cli/cmd_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/run/store_with_path_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`pub async fn add_paths( &self, id: Uuid, globs: &[String], ) -> Result<std::result::Result<(), String>>`

# Calls

- [get](../../../../functions/src/db/repository/memory/MemoryRepository/get.md)
- [validate_glob](../../../../functions/src/memory/paths/validate_glob.md)
- [paths_of](../../../../functions/src/db/repository/memory/MemoryRepository/paths_of.md)
- [add_paths](../../../../functions/src/db/repository/memory/MemoryRepository/add_paths.md)
- [changed](../../../../functions/src/memory/MemoryStore/changed.md)

# Called by

- [cmd_memory](../../../../functions/src/cli/cmd_memory.md)
- [path_notes_cut_by_the_cap_are_sent_in_a_later_step](../../../../functions/src/llm/agent/service/run/path_notes_cut_by_the_cap_are_sent_in_a_later_step.md)
- [store_with_path_memory](../../../../functions/src/llm/agent/service/run/store_with_path_memory.md)
- [save_to_store](../../../../functions/src/llm/tools/save_memory/SaveMemoryTool/save_to_store.md)