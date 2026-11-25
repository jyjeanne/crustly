---
type: Rust Method
title: update_session_usage
resource: src/services/session.rs#L101-L119
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/services/session/SessionService/get_session_required
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/db/repository/session/SessionRepository/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceContext/pool
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/db/repository/session/SessionRepository/update
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/test_update_session_usage
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn update_session_usage(&self, id: Uuid, token_count: i32, cost: f64) -> Result<()>`

# Calls

- [get_session_required](../../../../../functions/src/services/session/SessionService/get_session_required.md)
- [new](../../../../../functions/src/db/repository/session/SessionRepository/new.md)
- [pool](../../../../../functions/src/services/ServiceContext/pool.md)
- [update](../../../../../functions/src/db/repository/session/SessionRepository/update.md)
- [context](../../../../../functions/src/services/ServiceManager/context.md)

# Called by

- [send_message](../../../../../functions/src/llm/agent/service/AgentService/send_message.md)
- [send_message_with_tools_inner](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [test_update_session_usage](../../../../../functions/src/services/session/test_update_session_usage.md)