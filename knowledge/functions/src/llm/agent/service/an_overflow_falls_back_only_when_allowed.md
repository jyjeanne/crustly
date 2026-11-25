---
type: Rust Function
title: an_overflow_falls_back_only_when_allowed
resource: src/llm/agent/service.rs#L3961-L3989
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/agent/service/test_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/session_for
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/ScriptedProvider/named
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/agent/service/AgentService/with_fallback
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/fallback_to
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/text_reply
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/ScriptedProvider/requests
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn an_overflow_falls_back_only_when_allowed()`

# Calls

- [test_context](../../../../../functions/src/llm/agent/service/test_context.md)
- [session_for](../../../../../functions/src/llm/agent/service/session_for.md)
- [new](../../../../../functions/src/llm/agent/service/ScriptedProvider/new.md)
- [named](../../../../../functions/src/llm/agent/service/ScriptedProvider/named.md)
- [new](../../../../../functions/src/llm/agent/service/AgentService/new.md)
- [with_fallback](../../../../../functions/src/llm/agent/service/AgentService/with_fallback.md)
- [fallback_to](../../../../../functions/src/llm/agent/service/fallback_to.md)
- [send_message_with_tools](../../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools.md)
- [text_reply](../../../../../functions/src/llm/agent/service/text_reply.md)
- [requests](../../../../../functions/src/llm/agent/service/ScriptedProvider/requests.md)