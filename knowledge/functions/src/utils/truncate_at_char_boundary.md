---
type: Rust Function
title: truncate_at_char_boundary
resource: src/utils/mod.rs#L10-L19
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/pdf_context/augment_message_with_pdf
  - functions/src/services/session/SessionService/end_session_with_summary
---

# Signature

`pub fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str`

# Calls

- [len](../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [augment_message_with_pdf](../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [end_session_with_summary](../../../functions/src/services/session/SessionService/end_session_with_summary.md)