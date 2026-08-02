---
type: Rust Method
title: parse_xml
resource: src/llm/tools/doc_parser.rs#L521-L560
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/from_str
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_xml(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [from_str](../../../../../../functions/src/config/secrets/SecretString/from_str.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)