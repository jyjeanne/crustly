---
type: Rust Function
title: app_with_extraction
resource: src/tui/app.rs#L6391-L6424
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/db/Database/connect_in_memory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/run_migrations
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/MemoryStore/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/memory/extraction/MemoryStore/with_extraction
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/set_memory
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/App/create_new_session
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/tui/app/quitting_writes_the_session_episode_before_exit
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/app/starting_a_new_session_ends_the_previous_one
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn app_with_extraction( dir: &std::path::Path, ) -> (App, Arc<crate::memory::MemoryStore>, Database, Uuid)`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [new](../../../../functions/src/llm/agent/service/AgentService/new.md)
- [new](../../../../functions/src/memory/MemoryStore/new.md)
- [with_extraction](../../../../functions/src/memory/extraction/MemoryStore/with_extraction.md)
- [set_memory](../../../../functions/src/tui/app/App/set_memory.md)
- [create_new_session](../../../../functions/src/tui/app/App/create_new_session.md)

# Called by

- [quitting_writes_the_session_episode_before_exit](../../../../functions/src/tui/app/quitting_writes_the_session_episode_before_exit.md)
- [starting_a_new_session_ends_the_previous_one](../../../../functions/src/tui/app/starting_a_new_session_ends_the_previous_one.md)