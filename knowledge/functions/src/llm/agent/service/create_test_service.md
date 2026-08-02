---
type: Rust Function
title: create_test_service
resource: src/llm/agent/service.rs#L2149-L2167
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
---

# Signature

`async fn create_test_service() -> (AgentService, Uuid)`

# Calls

- [run_migrations](../../../../../functions/src/db/Database/run_migrations.md)