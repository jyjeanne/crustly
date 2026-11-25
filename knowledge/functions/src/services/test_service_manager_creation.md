---
type: Rust Function
title: test_service_manager_creation
resource: src/services/mod.rs#L110-L119
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/services/create_test_pool
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/services/ServiceManager/sessions
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/messages
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/files
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/services/ServiceManager/context
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn test_service_manager_creation()`

# Calls

- [create_test_pool](../../../functions/src/services/create_test_pool.md)
- [new](../../../functions/src/services/ServiceManager/new.md)
- [sessions](../../../functions/src/services/ServiceManager/sessions.md)
- [messages](../../../functions/src/services/ServiceManager/messages.md)
- [files](../../../functions/src/services/ServiceManager/files.md)
- [context](../../../functions/src/services/ServiceManager/context.md)