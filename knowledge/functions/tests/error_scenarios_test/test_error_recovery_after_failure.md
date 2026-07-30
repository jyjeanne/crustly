---
type: Rust Function
title: test_error_recovery_after_failure
resource: tests/error_scenarios_test.rs#L290-L315
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/tests/error_scenarios_test/create_error_agent
  - functions/src/services/session/SessionService/get_session
---

# Signature

`async fn test_error_recovery_after_failure() -> Result<()>`

# Calls

- [create_error_agent](../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)