---
type: Rust Method
title: extract_metadata_from_core_xml
resource: src/llm/tools/doc_parser.rs#L375-L408
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
---

# Signature

`fn extract_metadata_from_core_xml(xml: &str) -> (Option<String>, Option<String>)`

# Calls

- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)

# Called by

- [parse_docx](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)