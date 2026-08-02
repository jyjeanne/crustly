---
type: Rust Function
title: extract_pdf_text
resource: src/llm/pdf_context.rs#L55-L60
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/pdf_context/augment_message_with_pdf
---

# Signature

`pub fn extract_pdf_text(path: &Path) -> Result<String, String>`

# Called by

- [augment_message_with_pdf](../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)