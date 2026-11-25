---
type: Rust Function
title: test_error_timeout
resource: tests/error_scenarios_test.rs#L153-L171
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/error_scenarios_test/create_error_agent
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/session/SessionService/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/session/SessionService/create_session
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/memory/vector_index/VectorIndex/contains
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_error_timeout() -> Result<()>`

# Calls

- [create_error_agent](../../../functions/tests/error_scenarios_test/create_error_agent.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [contains](../../../functions/src/memory/vector_index/VectorIndex/contains.md)