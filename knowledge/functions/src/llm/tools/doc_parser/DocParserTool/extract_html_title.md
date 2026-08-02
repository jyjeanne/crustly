---
type: Rust Method
title: extract_html_title
resource: src/llm/tools/doc_parser.rs#L488-L497
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_html
  - functions/src/llm/tools/doc_parser/test_extract_html_title
---

# Signature

`fn extract_html_title(html: &str) -> Option<String>`

# Called by

- [parse_html](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [test_extract_html_title](../../../../../../functions/src/llm/tools/doc_parser/test_extract_html_title.md)