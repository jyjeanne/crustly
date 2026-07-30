---
type: Rust Module
title: models
resource: src/db/models.rs#L1-L446
generated:
  by: okf-rs/0.2.0
relationships:
  imports:
  - external/chrono-datetime-utc
  - external/serde-deserialize-serialize
  - external/sqlx-fromrow
  - external/uuid-uuid
  - external/sqlx-row
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [Session](../../../classes/src/db/models/Session.md)
- [Message](../../../classes/src/db/models/Message.md)
- [File](../../../classes/src/db/models/File.md)
- [Attachment](../../../classes/src/db/models/Attachment.md)
- [ToolExecution](../../../classes/src/db/models/ToolExecution.md)
- [CompactionRecord](../../../classes/src/db/models/CompactionRecord.md)
- [Plan](../../../classes/src/db/models/Plan.md)
- [PlanTaskStatus](../../../classes/src/db/models/PlanTaskStatus.md)
- [as_str](../../../functions/src/db/models/PlanTaskStatus/as_str.md)
- [parse](../../../functions/src/db/models/PlanTaskStatus/parse.md)
- [is_incomplete](../../../functions/src/db/models/PlanTaskStatus/is_incomplete.md)
- [PlanTask](../../../classes/src/db/models/PlanTask.md)
- [task_index](../../../functions/src/db/models/PlanTask/task_index.md)
- [exec_status](../../../functions/src/db/models/PlanTask/exec_status.md)
- [interrupted_plan_from_tasks](../../../functions/src/db/models/interrupted_plan_from_tasks.md)
- [new](../../../functions/src/db/models/Session/new.md)
- [is_archived](../../../functions/src/db/models/Session/is_archived.md)
- [new](../../../functions/src/db/models/Message/new.md)
- [new](../../../functions/src/db/models/File/new.md)
- [from_row](../../../functions/src/db/models/Session/sqlx-fromrow-r-sqlx-sqlite-sqliterow/from_row.md)
- [from_row](../../../functions/src/db/models/Message/sqlx-fromrow-r-sqlx-sqlite-sqliterow/from_row.md)
- [from_row](../../../functions/src/db/models/File/sqlx-fromrow-r-sqlx-sqlite-sqliterow/from_row.md)
- [from_row](../../../functions/src/db/models/Plan/sqlx-fromrow-r-sqlx-sqlite-sqliterow/from_row.md)
- [from_row](../../../functions/src/db/models/PlanTask/sqlx-fromrow-r-sqlx-sqlite-sqliterow/from_row.md)
- [test_session_new](../../../functions/src/db/models/test_session_new.md)
- [test_message_new](../../../functions/src/db/models/test_message_new.md)
- [test_file_new](../../../functions/src/db/models/test_file_new.md)
- [test_session_archived](../../../functions/src/db/models/test_session_archived.md)

# Imports

- `chrono::{DateTime, Utc}`
- `serde::{Deserialize, Serialize}`
- `sqlx::FromRow`
- `uuid::Uuid`
- `sqlx::Row`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)