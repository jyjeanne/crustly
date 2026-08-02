---
type: Rust Function
title: run_loop_exits_immediately_when_should_quit_is_set
resource: src/tui/runner.rs#L188-L202
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/tui/runner/run_loop
---

# Signature

`async fn run_loop_exits_immediately_when_should_quit_is_set()`

# Calls

- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [run_loop](../../../../functions/src/tui/runner/run_loop.md)