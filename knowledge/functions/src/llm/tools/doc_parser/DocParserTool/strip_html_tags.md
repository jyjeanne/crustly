---
type: Rust Method
title: strip_html_tags
resource: src/llm/tools/doc_parser.rs#L444-L485
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_html
  - functions/src/llm/tools/doc_parser/test_strip_html_tags
---

# Signature

`fn strip_html_tags(html: &str) -> String`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [parse_html](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [test_strip_html_tags](../../../../../../functions/src/llm/tools/doc_parser/test_strip_html_tags.md)