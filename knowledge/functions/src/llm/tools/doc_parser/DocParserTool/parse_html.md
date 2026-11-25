---
type: Rust Method
title: parse_html
resource: src/llm/tools/doc_parser.rs#L466-L481
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/strip_html_tags
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn parse_html(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [strip_html_tags](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/strip_html_tags.md)
- [extract_html_title](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)