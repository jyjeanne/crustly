---
type: Rust Module
title: services
resource: src/services/mod.rs#L1-L120
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/pub-use-file-fileservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-message-messageservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-plan-planservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/pub-use-session-sessionservice
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-pool
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-sync-arc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-pool-poolext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-db-database
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [ServiceContext](../../classes/src/services/ServiceContext.md)
- [new](../../functions/src/services/ServiceContext/new.md)
- [pool](../../functions/src/services/ServiceContext/pool.md)
- [ServiceManager](../../classes/src/services/ServiceManager.md)
- [new](../../functions/src/services/ServiceManager/new.md)
- [sessions](../../functions/src/services/ServiceManager/sessions.md)
- [messages](../../functions/src/services/ServiceManager/messages.md)
- [files](../../functions/src/services/ServiceManager/files.md)
- [plans](../../functions/src/services/ServiceManager/plans.md)
- [context](../../functions/src/services/ServiceManager/context.md)
- [create_test_pool](../../functions/src/services/create_test_pool.md)
- [test_service_context_creation](../../functions/src/services/test_service_context_creation.md)
- [test_service_manager_creation](../../functions/src/services/test_service_manager_creation.md)

# Imports

- `pub use file::FileService`
- `pub use message::MessageService`
- `pub use plan::PlanService`
- `pub use session::SessionService`
- `crate::db::Pool`
- `std::sync::Arc`
- `super::*`
- `crate::db::{Pool, PoolExt}`
- `crate::db::Database`

# Member of

- [crustly](../../packages/crustly.md)