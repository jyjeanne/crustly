---
type: Rust Method
title: extract_html_title
resource: src/llm/tools/doc_parser.rs#L528-L537
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/memory/MemoryStore/find
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_extract_html_title
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_html_title(html: &str) -> Option<String>`

# Calls

- [find](../../../../../../functions/src/memory/MemoryStore/find.md)

# Called by

- [parse_html](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [test_extract_html_title](../../../../../../functions/src/llm/tools/doc_parser/test_extract_html_title.md)