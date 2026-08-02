---
type: Rust Method
title: extract_text_from_docx_xml
resource: src/llm/tools/doc_parser.rs#L334-L372
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
---

# Signature

`fn extract_text_from_docx_xml(xml: &str) -> String`

# Calls

- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [parse_docx](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)