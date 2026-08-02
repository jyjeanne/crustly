---
type: Rust Function
title: test_end_to_end_session_management
resource: tests/integration_test.rs#L222-L276
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/integration_test/create_test_agent
  - functions/src/services/message/MessageService/list_messages_for_session
  - functions/src/services/session/SessionService/list_sessions
---

# Signature

`async fn test_end_to_end_session_management() -> Result<()>`

# Calls

- [create_test_agent](../../../functions/tests/integration_test/create_test_agent.md)
- [list_messages_for_session](../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [list_sessions](../../../functions/src/services/session/SessionService/list_sessions.md)