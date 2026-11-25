---
type: Rust Method
title: extract_text_from_docx_xml
resource: src/llm/tools/doc_parser.rs#L354-L393
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_reads_paragraphs_and_unescapes_entities
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_ignores_text_outside_w_t
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn extract_text_from_docx_xml(xml: &str) -> String`

# Calls

- [resolve_general_ref](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/resolve_general_ref.md)

# Called by

- [parse_docx](../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [test_extract_text_from_docx_xml_reads_paragraphs_and_unescapes_entities](../../../../../../functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_reads_paragraphs_and_unescapes_entities.md)
- [test_extract_text_from_docx_xml_ignores_text_outside_w_t](../../../../../../functions/src/llm/tools/doc_parser/test_extract_text_from_docx_xml_ignores_text_outside_w_t.md)