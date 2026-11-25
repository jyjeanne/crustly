---
type: Rust Function
title: context_with_file
resource: src/llm/tools/doc_parser.rs#L633-L643
tags:
- test
visibility: private
generated:
  by: okf-rs/0.8.0
relationships:
  calls:
  - target: functions/src/llm/tools/trait/ToolExecutionContext/new
    resolved_by: tree-sitter-receiver
    confidence: semantic
  - target: functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory
    resolved_by: tree-sitter-receiver
    confidence: semantic
  called_by:
  - target: functions/src/llm/tools/doc_parser/test_parse_text_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_parse_markdown_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_parse_json_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_parse_html_file
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_max_chars_truncation
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_unsupported_format
    resolved_by: tree-sitter
    confidence: exact
  - target: functions/src/llm/tools/doc_parser/test_parse_xml_file
    resolved_by: tree-sitter
    confidence: exact
---

# Signature

`fn context_with_file(name: &str, content: &str) -> (TempDir, PathBuf, ToolExecutionContext)`

# Calls

- [new](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/new.md)
- [with_working_directory](../../../../../functions/src/llm/tools/trait/ToolExecutionContext/with_working_directory.md)

# Called by

- [test_parse_text_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_text_file.md)
- [test_parse_markdown_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_markdown_file.md)
- [test_parse_json_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_json_file.md)
- [test_parse_html_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_html_file.md)
- [test_max_chars_truncation](../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation.md)
- [test_max_chars_truncation_does_not_panic_on_multibyte_text](../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text.md)
- [test_unsupported_format](../../../../../functions/src/llm/tools/doc_parser/test_unsupported_format.md)
- [test_parse_xml_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_xml_file.md)