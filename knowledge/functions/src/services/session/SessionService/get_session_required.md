---
type: Rust Method
title: get_session_required
resource: src/services/session.rs#L59-L63
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/services/session/SessionService/get_session
  called_by:
  - functions/src/services/session/SessionService/update_session_title
  - functions/src/services/session/SessionService/update_session_usage
  - functions/src/services/session/test_get_session_required
  - functions/src/services/session/test_update_session_title
  - functions/src/services/session/test_update_session_usage
  - functions/src/services/session/test_archive_unarchive_session
---

# Signature

`pub async fn get_session_required(&self, id: Uuid) -> Result<Session>`

# Calls

- [get_session](../../../../../functions/src/services/session/SessionService/get_session.md)

# Called by

- [update_session_title](../../../../../functions/src/services/session/SessionService/update_session_title.md)
- [update_session_usage](../../../../../functions/src/services/session/SessionService/update_session_usage.md)
- [test_get_session_required](../../../../../functions/src/services/session/test_get_session_required.md)
- [test_update_session_title](../../../../../functions/src/services/session/test_update_session_title.md)
- [test_update_session_usage](../../../../../functions/src/services/session/test_update_session_usage.md)
- [test_archive_unarchive_session](../../../../../functions/src/services/session/test_archive_unarchive_session.md)