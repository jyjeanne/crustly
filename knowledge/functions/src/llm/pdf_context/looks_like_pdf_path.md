---
type: Rust Function
title: looks_like_pdf_path
resource: src/llm/pdf_context.rs#L28-L49
generated:
  by: okf-rs/0.2.0
relationships:
  called_by:
  - functions/src/llm/pdf_context/augment_message_with_pdf
  - functions/src/llm/pdf_context/detects_absolute_pdf_token
  - functions/src/llm/pdf_context/detects_relative_pdf_token
  - functions/src/llm/pdf_context/returns_none_for_missing_file
  - functions/src/llm/pdf_context/returns_none_when_no_pdf
---

# Signature

`pub fn looks_like_pdf_path(text: &str, cwd: &Path) -> Option<PathBuf>`

# Called by

- [augment_message_with_pdf](../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [detects_absolute_pdf_token](../../../../functions/src/llm/pdf_context/detects_absolute_pdf_token.md)
- [detects_relative_pdf_token](../../../../functions/src/llm/pdf_context/detects_relative_pdf_token.md)
- [returns_none_for_missing_file](../../../../functions/src/llm/pdf_context/returns_none_for_missing_file.md)
- [returns_none_when_no_pdf](../../../../functions/src/llm/pdf_context/returns_none_when_no_pdf.md)