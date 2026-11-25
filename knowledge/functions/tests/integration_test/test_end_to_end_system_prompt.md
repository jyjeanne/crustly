---
type: Rust Function
title: test_end_to_end_system_prompt
resource: tests/integration_test.rs#L383-L408
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/tests/integration_test/MockProvider/single_response
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/Database/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_system_prompt
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

`async fn test_end_to_end_system_prompt() -> Result<()>`

# Calls

- [single_response](../../../functions/tests/integration_test/MockProvider/single_response.md)
- [new](../../../functions/src/services/ServiceContext/new.md)
- [pool](../../../functions/src/db/Database/pool.md)
- [new](../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_system_prompt](../../../functions/src/llm/agent/service/AgentService/with_system_prompt.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [send_message](../../../functions/src/llm/agent/service/AgentService/send_message.md)