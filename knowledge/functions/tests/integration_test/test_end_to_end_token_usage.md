---
type: Rust Function
title: test_end_to_end_token_usage
resource: tests/integration_test.rs#L345-L380
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/tests/integration_test/create_test_agent
  - functions/src/services/message/MessageService/list_messages_for_session
  - functions/src/services/session/SessionService/get_session
---

# Signature

`async fn test_end_to_end_token_usage() -> Result<()>`

# Calls

- [create_test_agent](../../../functions/tests/integration_test/create_test_agent.md)
- [list_messages_for_session](../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)