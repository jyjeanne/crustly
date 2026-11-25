---
type: Rust Module
title: session
resource: src/services/session.rs#L1-L384
generated:
  by: okf-rs/0.8.0
relationships:
  imports:
  - target: external/crate-db-models-session-repository-sessionlistoptions-sessionrepository
    resolved_by: tree-sitter
    confidence: exact
  - target: external/crate-services-servicecontext
    resolved_by: tree-sitter
    confidence: exact
  - target: external/anyhow-context-result
    resolved_by: tree-sitter
    confidence: exact
  - target: external/chrono-utc
    resolved_by: tree-sitter
    confidence: exact
  - target: external/uuid-uuid
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
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
    repository::{SessionListOptions, SessionRepository},
}`
- `crate::services::ServiceContext`
- `anyhow::{Context, Result}`
- `chrono::Utc`
- `uuid::Uuid`
- `super::*`
- `crate::db::Database`

# Member of

- [crustly](../../../packages/crustly.md)