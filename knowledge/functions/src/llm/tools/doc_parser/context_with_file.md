---
type: Rust Function
title: context_with_file
resource: src/llm/tools/doc_parser.rs#L574-L584
visibility: private
generated:
  by: okf-rs/0.3.0
relationships:
  called_by:
  - functions/src/llm/tools/doc_parser/test_parse_text_file
  - functions/src/llm/tools/doc_parser/test_parse_markdown_file
  - functions/src/llm/tools/doc_parser/test_parse_json_file
  - functions/src/llm/tools/doc_parser/test_parse_html_file
  - functions/src/llm/tools/doc_parser/test_max_chars_truncation
  - functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text
  - functions/src/llm/tools/doc_parser/test_unsupported_format
---

# Signature

`fn context_with_file(name: &str, content: &str) -> (TempDir, PathBuf, ToolExecutionContext)`

# Called by

- [test_parse_text_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_text_file.md)
- [test_parse_markdown_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_markdown_file.md)
- [test_parse_json_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_json_file.md)
- [test_parse_html_file](../../../../../functions/src/llm/tools/doc_parser/test_parse_html_file.md)
- [test_max_chars_truncation](../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation.md)
- [test_max_chars_truncation_does_not_panic_on_multibyte_text](../../../../../functions/src/llm/tools/doc_parser/test_max_chars_truncation_does_not_panic_on_multibyte_text.md)
- [test_unsupported_format](../../../../../functions/src/llm/tools/doc_parser/test_unsupported_format.md)