---
type: Rust Function
title: run_loop_exits_immediately_when_should_quit_is_set
resource: src/tui/runner.rs#L193-L207
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
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
  - target: functions/src/tui/runner/run_loop
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn run_loop_exits_immediately_when_should_quit_is_set()`

# Calls

- [connect_in_memory](../../../../functions/src/db/Database/connect_in_memory.md)
- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [new](../../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../../functions/src/db/Database/pool.md)
- [new](../../../../functions/src/llm/agent/service/AgentService/new.md)
- [run_loop](../../../../functions/src/tui/runner/run_loop.md)