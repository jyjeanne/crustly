---
type: Rust Module
title: session
resource: src/services/session.rs#L1-L440
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/crate-db-models-session-repository-episodicmemoryrepository-sessionlistoptions-sessionrepository
  - external/crate-llm-agent-memory-episodicmemory
  - external/crate-llm-provider-types-message
  - external/crate-services-servicecontext
  - external/anyhow-context-result
  - external/chrono-utc
  - external/uuid-uuid
  - external/crate-llm-agent-context-token-count
  - external/crate-llm-provider-types-contentblock
  - external/crate-llm-provider-types-role
  - external/super
  - external/crate-db-database
  member_of:
  - packages/crustly
---

# Contains

- [SessionService](../../../classes/src/services/session/SessionService.md)
- [new](../../../functions/src/services/session/SessionService/new.md)
- [create_session](../../../functions/src/services/session/SessionService/create_session.md)
- [get_session](../../../functions/src/services/session/SessionService/get_session.md)
- [get_session_required](../../../functions/src/services/session/SessionService/get_session_required.md)
- [list_sessions](../../../functions/src/services/session/SessionService/list_sessions.md)
- [update_session](../../../functions/src/services/session/SessionService/update_session.md)
- [update_session_title](../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../functions/src/services/session/SessionService/update_session_usage.md)
- [archive_session](../../../functions/src/services/session/SessionService/archive_session.md)
- [unarchive_session](../../../functions/src/services/session/SessionService/unarchive_session.md)
- [delete_session](../../../functions/src/services/session/SessionService/delete_session.md)
- [get_most_recent_session](../../../functions/src/services/session/SessionService/get_most_recent_session.md)
- [count_sessions](../../../functions/src/services/session/SessionService/count_sessions.md)
- [count_archived_sessions](../../../functions/src/services/session/SessionService/count_archived_sessions.md)
- [end_session_with_summary](../../../functions/src/services/session/SessionService/end_session_with_summary.md)
- [create_test_service](../../../functions/src/services/session/create_test_service.md)
- [test_create_session](../../../functions/src/services/session/test_create_session.md)
- [test_get_session](../../../functions/src/services/session/test_get_session.md)
- [test_get_session_required](../../../functions/src/services/session/test_get_session_required.md)
- [test_update_session_title](../../../functions/src/services/session/test_update_session_title.md)
- [test_update_session_usage](../../../functions/src/services/session/test_update_session_usage.md)
- [test_archive_unarchive_session](../../../functions/src/services/session/test_archive_unarchive_session.md)
- [test_delete_session](../../../functions/src/services/session/test_delete_session.md)
- [test_list_sessions](../../../functions/src/services/session/test_list_sessions.md)
- [test_get_most_recent_session](../../../functions/src/services/session/test_get_most_recent_session.md)
- [test_count_sessions](../../../functions/src/services/session/test_count_sessions.md)

# Imports

- `crate::db::{
    models::Session,
    repository::{EpisodicMemoryRepository, SessionListOptions, SessionRepository},
}`
- `crate::llm::agent::memory::EpisodicMemory`
- `crate::llm::provider::types::Message`
- `crate::services::ServiceContext`
- `anyhow::{Context, Result}`
- `chrono::Utc`
- `uuid::Uuid`
- `crate::llm::agent::context::token_count`
- `crate::llm::provider::types::ContentBlock`
- `crate::llm::provider::types::Role`
- `super::*`
- `crate::db::Database`

# Member of

- [crustly](../../../packages/crustly.md)