---
type: Rust Function
title: augment_message_with_pdf
resource: src/llm/pdf_context.rs#L78-L127
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/pdf_context/looks_like_pdf_path
  - functions/src/llm/pdf_context/extract_pdf_text
  - functions/src/config/secrets/SecretString/is_empty
  - functions/src/utils/truncate_at_char_boundary
  - functions/src/config/secrets/SecretString/len
  called_by:
  - functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
  - functions/src/llm/agent/service/AgentService/prepare_message_context
  - functions/src/llm/pdf_context/augment_returns_original_when_no_pdf
  - functions/src/llm/pdf_context/augment_returns_original_on_extraction_failure
---

# Signature

`pub async fn augment_message_with_pdf(message: &str, cwd: &Path) -> String`

# Calls

- [looks_like_pdf_path](../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)
- [extract_pdf_text](../../../../functions/src/llm/pdf_context/extract_pdf_text.md)
- [is_empty](../../../../functions/src/config/secrets/SecretString/is_empty.md)
- [truncate_at_char_boundary](../../../../functions/src/utils/truncate_at_char_boundary.md)
- [len](../../../../functions/src/config/secrets/SecretString/len.md)

# Called by

- [send_message_with_tools_inner](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [augment_returns_original_when_no_pdf](../../../../functions/src/llm/pdf_context/augment_returns_original_when_no_pdf.md)
- [augment_returns_original_on_extraction_failure](../../../../functions/src/llm/pdf_context/augment_returns_original_on_extraction_failure.md)