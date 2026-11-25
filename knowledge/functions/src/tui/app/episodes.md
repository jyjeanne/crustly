---
type: Rust Function
title: episodes
resource: src/tui/app.rs#L6426-L6438
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/project/identify
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/list
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/tui/app/quitting_writes_the_session_episode_before_exit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/starting_a_new_session_ends_the_previous_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn episodes( store: &crate::memory::MemoryStore, dir: &std::path::Path, ) -> Vec<crate::memory::model::Memory>`

# Calls

- [identify](../../../../functions/src/memory/project/identify.md)
- [list](../../../../functions/src/memory/MemoryStore/list.md)

# Called by

- [quitting_writes_the_session_episode_before_exit](../../../../functions/src/tui/app/quitting_writes_the_session_episode_before_exit.md)
- [starting_a_new_session_ends_the_previous_one](../../../../functions/src/tui/app/starting_a_new_session_ends_the_previous_one.md)