---
type: Rust Function
title: truncate_at_char_boundary
resource: src/utils/mod.rs#L10-L19
generated:
  by: okf-rs/0.8.0
relationships:
  called_by:
  - target: functions/src/llm/agent/compaction/elide_tool_results
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/pdf_context/augment_message_with_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/http/HttpClientTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/tui/render/render_approval_input_summary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_ascii_at_exact_boundary
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_multibyte_does_not_panic
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_empty_and_zero
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`pub fn truncate_at_char_boundary(s: &str, max_bytes: usize) -> &str`

# Called by

- [elide_tool_results](../../../functions/src/llm/agent/compaction/elide_tool_results.md)
- [augment_message_with_pdf](../../../functions/src/llm/pdf_context/augment_message_with_pdf.md)
- [execute](../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)
- [execute](../../../functions/src/llm/tools/http/HttpClientTool/tool/execute.md)
- [render_approval_input_summary](../../../functions/src/tui/render/render_approval_input_summary.md)
- [truncate_ascii_at_exact_boundary](../../../functions/src/utils/truncate_ascii_at_exact_boundary.md)
- [truncate_multibyte_does_not_panic](../../../functions/src/utils/truncate_multibyte_does_not_panic.md)
- [truncate_empty_and_zero](../../../functions/src/utils/truncate_empty_and_zero.md)