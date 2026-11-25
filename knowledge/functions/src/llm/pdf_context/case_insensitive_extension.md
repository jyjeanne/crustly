---
type: Rust Function
title: case_insensitive_extension
resource: src/llm/pdf_context.rs#L160-L168
tags:
- test
visibility: private
generated:
  by: okf-rs/0.7.0
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

`fn case_insensitive_extension()`

# Calls

- [write](../../../../functions/src/memory/project/write.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [looks_like_pdf_path](../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)