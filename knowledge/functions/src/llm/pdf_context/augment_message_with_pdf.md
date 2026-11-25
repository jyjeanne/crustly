---
type: Rust Function
title: augment_message_with_pdf
resource: src/llm/pdf_context.rs#L78-L127
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/pdf_context/looks_like_pdf_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/extract_pdf_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_at_char_boundary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/agent/service/AgentService/send_message_with_tools_inner
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/agent/service/AgentService/prepare_message_context
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_returns_original_when_no_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_returns_original_on_extraction_failure
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub async fn augment_message_with_pdf(message: &str, cwd: &Path) -> String`

# Calls

- [looks_like_pdf_path](../../../../functions/src/llm/pdf_context/looks_like_pdf_path.md)
- [extract_pdf_text](../../../../functions/src/llm/pdf_context/extract_pdf_text.md)
- [truncate_at_char_boundary](../../../../functions/src/utils/truncate_at_char_boundary.md)
- [display](../../../../functions/src/llm/tools/skill/SkillSource/display.md)

# Called by

- [send_message_with_tools_inner](../../../../functions/src/llm/agent/service/AgentService/send_message_with_tools_inner.md)
- [prepare_message_context](../../../../functions/src/llm/agent/service/AgentService/prepare_message_context.md)
- [augment_returns_original_when_no_pdf](../../../../functions/src/llm/pdf_context/augment_returns_original_when_no_pdf.md)
- [augment_returns_original_on_extraction_failure](../../../../functions/src/llm/pdf_context/augment_returns_original_on_extraction_failure.md)