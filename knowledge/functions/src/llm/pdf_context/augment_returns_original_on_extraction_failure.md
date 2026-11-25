---
type: Rust Function
title: augment_returns_original_on_extraction_failure
resource: src/llm/pdf_context.rs#L202-L214
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn augment_returns_original_on_extraction_failure()`

# Calls

- [augment_message_with_pdf](../../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)