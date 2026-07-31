---
type: Rust Function
title: test_end_to_end_multi_turn_conversation
resource: tests/integration_test.rs#L172-L219
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/tests/integration_test/create_test_agent
  - functions/src/services/message/MessageService/list_messages_for_session
---

# Signature

`async fn test_end_to_end_multi_turn_conversation() -> Result<()>`

# Calls

- [create_test_agent](../../../functions/tests/integration_test/create_test_agent.md)
- [list_messages_for_session](../../../functions/src/services/message/MessageService/list_messages_for_session.md)