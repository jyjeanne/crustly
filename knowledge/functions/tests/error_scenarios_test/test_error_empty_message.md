---
type: Rust Function
title: test_error_empty_message
resource: tests/error_scenarios_test.rs#L235-L258
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn test_error_empty_message() -> Result<()>`

# Calls

- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [send_message](../../../functions/src/llm/agent/service/AgentService/send_message.md)