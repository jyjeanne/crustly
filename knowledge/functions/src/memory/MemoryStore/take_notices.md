---
type: Rust Method
title: take_notices
resource: src/memory/mod.rs#L828-L833
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/cli/cmd_run
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/a_refused_global_save_stores_nothing
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/finish_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/announce_memory_saves
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/saves_are_announced_once
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/tests/memory_store_test/a_secret_is_rejected_and_nothing_is_stored
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn take_notices(&self) -> Vec<String>`

# Called by

- [cmd_run](../../../../functions/src/cli/cmd_run.md)
- [a_refused_global_save_stores_nothing](../../../../functions/src/llm/agent/service/a_refused_global_save_stores_nothing.md)
- [database_save_is_stored_announced_and_not_written_to_the_file](../../../../functions/src/llm/tools/save_memory/database_save_is_stored_announced_and_not_written_to_the_file.md)
- [finish_memory](../../../../functions/src/tui/app/App/finish_memory.md)
- [announce_memory_saves](../../../../functions/src/tui/app/App/announce_memory_saves.md)
- [a_save_during_a_response_is_announced_in_the_chat](../../../../functions/src/tui/app/a_save_during_a_response_is_announced_in_the_chat.md)
- [saves_are_announced_once](../../../../functions/tests/memory_store_test/saves_are_announced_once.md)
- [a_secret_is_rejected_and_nothing_is_stored](../../../../functions/tests/memory_store_test/a_secret_is_rejected_and_nothing_is_stored.md)