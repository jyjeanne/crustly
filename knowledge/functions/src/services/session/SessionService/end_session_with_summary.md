---
type: Rust Method
title: end_session_with_summary
resource: src/services/session.rs#L186-L233
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/utils/truncate_at_char_boundary
  - functions/src/llm/agent/context/token_count
---

# Signature

`pub async fn end_session_with_summary( &self, session_id: Uuid, messages: Vec<Message>, files_touched: Vec<String>, ) -> Result<()>`

# Calls

- [truncate_at_char_boundary](../../../../../functions/src/utils/truncate_at_char_boundary.md)
- [token_count](../../../../../functions/src/llm/agent/context/token_count.md)