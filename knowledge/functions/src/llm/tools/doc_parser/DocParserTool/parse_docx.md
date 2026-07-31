---
type: Rust Method
title: parse_docx
resource: src/llm/tools/doc_parser.rs#L286-L331
visibility: private
generated:
  by: okf-rs/0.2.0
relationships:
  calls:
  - functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml
  - functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml
  called_by:
  - functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
---

# Signature

`async fn parse_docx(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [extract_text_from_docx_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml.md)
- [extract_metadata_from_core_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)