---
type: Rust Method
title: execute
resource: src/llm/tools/doc_parser.rs#L102-L219
visibility: private
generated:
  by: okf-rs/0.7.0
relationships:
  calls:
  - target: functions/src/llm/tools/sandbox/check_path
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/error
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_pdf
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_docx
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_html
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_json
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/DocParserTool/parse_xml
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/trait/ToolResult/success
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolResult/with_metadata
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/skill/SkillSource/display
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/utils/truncate_at_char_boundary
    resolved_by: tree-sitter
    confidence: exact
  called_by:
  - target: functions/src/llm/tools/doc_parser/test_parse_text_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_parse_markdown_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_parse_json_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_parse_html_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_max_chars_truncation
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_unsupported_format
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_nonexistent_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/doc_parser/test_parse_xml_file
    resolved_by: tree-sitter-receiver
    confidence: semantic
---

# Signature

`async fn execute(&self, input: Value, context: &ToolExecutionContext) -> Result<ToolResult>`

# Calls

- [check_path](../../../../../../../functions/src/llm/tools/sandbox/check_path.md)
- [error](../../../../../../../functions/src/llm/tools/trait/ToolResult/error.md)
- [parse_pdf](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_pdf.md)
- [parse_docx](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_docx.md)
- [parse_text](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_text.md)
- [parse_html](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_html.md)
- [parse_json](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_json.md)
- [parse_xml](../../../../../../../functions/src/llm/tools/doc_parser/DocParserTool/parse_xml.md)
- [success](../../../../../../../functions/src/llm/tools/trait/ToolResult/success.md)
- [with_metadata](../../../../../../../functions/src/llm/tools/trait/ToolResult/with_metadata.md)
- [display](../../../../../../../functions/src/llm/tools/skill/SkillSource/display.md)
- [truncate_at_char_boundary](../../../../../../../functions/src/utils/truncate_at_char_boundary.md)

# Called by

- [test_parse_text_file](../../../../../../../functions/src/llm/tools/doc_parser/test_parse_text_file.md)
- [test_parse_markdown_file](../../../../../../../functions/src/llm/tools/doc_parser/test_parse_markdown_file.md)
- [test_parse_json_file](../../../../../../../functions/src/llm/tools/doc_parser/test_parse_json_file.md)
- [test_parse_html_file](../../../../../../../functions/src/llm/tools/doc_parser/test_parse_html_file.md)
- [test_max_chars_truncation](../../../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation.md)
- [test_max_chars_truncation_does_not_panic_on_multibyte_text](../../../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text.md)
- [test_unsupported_format](../../../../../../../functions/src/llm/tools/doc_parser/test_unsupported_format.md)
- [test_nonexistent_file](../../../../../../../functions/src/llm/tools/doc_parser/test_nonexistent_file.md)
- [test_path_outside_working_directory_is_denied](../../../../../../../functions/src/llm/tools/doc_parser/test_path_outside_working_directory_is_denied.md)
- [test_parse_xml_file](../../../../../../../functions/src/llm/tools/doc_parser/test_parse_xml_file.md)