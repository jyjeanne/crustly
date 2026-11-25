---
type: Rust Function
title: looks_like_pdf_path
resource: src/llm/pdf_context.rs#L28-L49
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/detects_absolute_pdf_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/detects_relative_pdf_token
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/returns_none_for_missing_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/returns_none_when_no_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/case_insensitive_extension
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/strips_surrounding_quotes
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn looks_like_pdf_path(text: &str, cwd: &Path) -> Option<PathBuf>`

# Called by

- [augment_message_with_pdf](../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [detects_absolute_pdf_token](../../../../functions/src/llm/pdf_context/detects_absolute_pdf_token.md)
- [detects_relative_pdf_token](../../../../functions/src/llm/pdf_context/detects_relative_pdf_token.md)
- [returns_none_for_missing_file](../../../../functions/src/llm/pdf_context/returns_none_for_missing_file.md)
- [returns_none_when_no_pdf](../../../../functions/src/llm/pdf_context/returns_none_when_no_pdf.md)
- [case_insensitive_extension](../../../../functions/src/llm/pdf_context/case_insensitive_extension.md)
- [strips_surrounding_quotes](../../../../functions/src/llm/pdf_context/strips_surrounding_quotes.md)