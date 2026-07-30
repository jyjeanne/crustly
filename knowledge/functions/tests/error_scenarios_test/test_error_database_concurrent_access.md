---
type: Rust Function
title: test_error_database_concurrent_access
resource: tests/error_scenarios_test.rs#L261-L287
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session
---

# Signature

`async fn test_error_database_concurrent_access() -> Result<()>`

# Calls

- [get_session](../../../functions/src/services/session/SessionService/get_session.md)