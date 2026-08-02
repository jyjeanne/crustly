---
type: Rust Method
title: parse_pdf
resource: src/llm/tools/doc_parser.rs#L231-L283
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  calls:
  - functions/src/config/secrets/SecretString/len
  - functions/src/config/secrets/SecretString/is_empty
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_pdf( &self, path: &Path, input: &DocParserInput, ) -> Result<(String, ParsedMetadata)>`

# Calls

- [len](../../../../../../functions/src/config/secrets/SecretString/len.md)
- [is_empty](../../../../../../functions/src/config/secrets/SecretString/is_empty.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)