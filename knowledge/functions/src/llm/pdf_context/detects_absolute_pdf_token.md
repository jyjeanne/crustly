---
type: Rust Function
title: detects_absolute_pdf_token
resource: src/llm/pdf_context.rs#L136-L145
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/memory/project/write
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/looks_like_pdf_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn detects_absolute_pdf_token()`

# Calls

- [write](../../../../functions/src/memory/project/write.md)
- [looks_like_pdf_path](../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)