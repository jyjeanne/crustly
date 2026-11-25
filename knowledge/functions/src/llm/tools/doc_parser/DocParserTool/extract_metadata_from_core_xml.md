---
type: Rust Method
title: extract_metadata_from_core_xml
resource: src/llm/tools/doc_parser.rs#L396-L448
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml_missing_fields
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_metadata_from_core_xml(xml: &str) -> (Option<String>, Option<String>)`

# Calls

- [resolve_general_ref](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref.md)

# Called by

- [parse_docx](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [test_extract_metadata_from_core_xml](../../../../../../functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml.md)
- [test_extract_metadata_from_core_xml_missing_fields](../../../../../../functions/src/llm/tools/doc_parser/test_extract_metadata_from_core_xml_missing_fields.md)