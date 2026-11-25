---
type: Rust Method
title: parse_docx
resource: src/llm/tools/doc_parser.rs#L286-L331
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/tests/memory_acceptance_test/open
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/tool/execute
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`async fn parse_docx(&self, path: &Path) -> Result<(String, ParsedMetadata)>`

# Calls

- [open](../../../../../../functions/tests/memory_acceptance_test/open.md)
- [extract_text_from_docx_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml.md)
- [extract_metadata_from_core_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml.md)

# Called by

- [execute](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/tool/execute.md)