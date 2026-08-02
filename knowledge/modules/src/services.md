---
type: Rust Module
title: services
resource: src/services/mod.rs#L1-L120
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/pub-use-file-fileservice
  - external/pub-use-message-messageservice
  - external/pub-use-plan-planservice
  - external/pub-use-session-sessionservice
  - external/crate-db-pool
  - external/std-sync-arc
  - external/super
  - external/crate-db-pool-poolext
  - external/crate-db-database
  member_of:
  - packages/crustly
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