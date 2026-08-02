---
type: Rust Function
title: test_end_to_end_cost_tracking
resource: tests/integration_test.rs#L279-L323
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/integration_test/create_test_agent
  - functions/src/services/session/SessionService/get_session
---

# Signature

`async fn test_end_to_end_cost_tracking() -> Result<()>`

# Calls

- [create_test_agent](../../../functions/tests/integration_test/create_test_agent.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)