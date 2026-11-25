---
type: Rust Method
title: resolve_general_ref
resource: src/llm/tools/doc_parser.rs#L345-L351
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_xml
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn resolve_general_ref(r: &quick_xml::events::BytesRef<'_>) -> Option<String>`

# Called by

- [extract_text_from_docx_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_text_from_docx_xml.md)
- [extract_metadata_from_core_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/extract_metadata_from_core_xml.md)
- [parse_xml](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)