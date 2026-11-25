---
type: Rust Method
title: strip_html_tags
resource: src/llm/tools/doc_parser.rs#L484-L525
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_strip_html_tags
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn strip_html_tags(html: &str) -> String`

# Called by

- [parse_html](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [test_strip_html_tags](../../../../../../functions/src/llm/tools/doc_parser/test_strip_html_tags.md)