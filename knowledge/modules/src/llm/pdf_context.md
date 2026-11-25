---
type: Rust Module
title: pdf_context
resource: src/llm/pdf_context.rs#L1-L215
generated:
  by: okf-rs/0.7.0
relationships:
  imports:
  - target: external/std-path-path-pathbuf
    resolved_by: tree-sitter
    confidence: exact
  - target: external/super
    resolved_by: tree-sitter
    confidence: exact
  - target: external/std-io-write
    resolved_by: tree-sitter
    confidence: exact
  - target: external/tempfile-namedtempfile
    resolved_by: tree-sitter
    confidence: exact
  member_of:
  - target: packages/crustly
    resolved_by: tree-sitter
    confidence: exact
---

# Contains

- [looks_like_pdf_path](../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)
- [extract_pdf_text](../../../functions/src/llm/pdf_context/extract_pdf_text.md)
- [augment_message_with_pdf](../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [detects_absolute_pdf_token](../../../functions/src/llm/pdf_context/detects_absolute_pdf_token.md)
- [detects_relative_pdf_token](../../../functions/src/llm/pdf_context/detects_relative_pdf_token.md)
- [case_insensitive_extension](../../../functions/src/llm/pdf_context/case_insensitive_extension.md)
- [strips_surrounding_quotes](../../../functions/src/llm/pdf_context/strips_surrounding_quotes.md)
- [returns_none_for_missing_file](../../../functions/src/llm/pdf_context/returns_none_for_missing_file.md)
- [returns_none_when_no_pdf](../../../functions/src/llm/pdf_context/returns_none_when_no_pdf.md)
- [augment_returns_original_when_no_pdf](../../../functions/src/llm/pdf_context/augment_returns_original_when_no_pdf.md)
- [augment_returns_original_on_extraction_failure](../../../functions/src/llm/pdf_context/augment_returns_original_on_extraction_failure.md)

# Imports

- `std::path::{Path, PathBuf}`
- `super::*`
- `std::io::Write`
- `tempfile::NamedTempFile`

# Member of

- [crustly](../../../packages/crustly.md)