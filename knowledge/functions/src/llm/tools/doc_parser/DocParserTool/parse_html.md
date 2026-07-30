---
type: Rust Method
title: parse_html
resource: src/llm/tools/doc_parser.rs#L426-L441
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/doc_parser/DocParserTool/strip_html_tags
  - functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_html(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [strip_html_tags](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/strip_html_tags.md)
- [extract_html_title](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_html_title.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)