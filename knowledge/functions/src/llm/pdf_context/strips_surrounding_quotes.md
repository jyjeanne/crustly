---
type: Rust Function
title: strips_surrounding_quotes
resource: src/llm/pdf_context.rs#L171-L179
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
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/looks_like_pdf_path
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strips_surrounding_quotes()`

# Calls

- [write](../../../../functions/src/memory/project/write.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [looks_like_pdf_path](../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)