---
type: Rust Function
title: create_then_update_survives_a_file_backed_wal_pool
resource: src/services/message.rs#L229-L259
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/db/Database/run_migrations
  - functions/src/services/message/MessageService/create_message
  - functions/src/services/message/MessageService/update_message_usage
---

# Signature

`async fn create_then_update_survives_a_file_backed_wal_pool()`

# Calls

- [run_migrations](../../../../functions/src/db/Database/run_migrations.md)
- [create_message](../../../../functions/src/services/message/MessageService/create_message.md)
- [update_message_usage](../../../../functions/src/services/message/MessageService/update_message_usage.md)