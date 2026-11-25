---
type: Rust Function
title: test_end_to_end_token_usage
resource: tests/integration_test.rs#L345-L380
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/integration_test/create_test_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/message/MessageService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/message/MessageService/list_messages_for_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/get_session
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_end_to_end_token_usage() -> Result<()>`

# Calls

- [create_test_agent](../../../functions/tests/integration_test/create_test_agent.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [new](../../../functions/src/services/message/MessageService/new.md)
- [list_messages_for_session](../../../functions/src/services/message/MessageService/list_messages_for_session.md)
- [find](../../../functions/src/memory/MemoryStore/find.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)